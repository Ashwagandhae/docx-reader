use crate::document::Document;
use crate::document::OutlinePara;
use crate::document::Para;

use serde::{Deserialize, Serialize};
use std::cmp;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;
use tauri::Window;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Query {
  pub text: String,
  pub match_case: bool,
  pub only_outline: bool,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct SearchResult {
  pub link: usize,
  pub index: usize,
  pub para: Para,
  pub query_index: usize,
}

pub struct Paras(pub Mutex<HashMap<String, Vec<Para>>>);
pub struct OutlineParas(pub Mutex<HashMap<String, Vec<OutlinePara>>>);
pub struct SearchResultsState {
  pub results: Vec<SearchResult>,
  pub last_query: Option<Query>,
  pub para_texts: Vec<String>,
}

pub struct SearchResults(pub Mutex<HashMap<String, SearchResultsState>>);

#[tauri::command]
pub fn load_file(
  path: String,
  paras: State<Paras>,
  outline_paras: State<OutlineParas>,
  search_results: State<SearchResults>,
  window: Window,
) -> bool {
  let label = window.label();
  let mut paras_dict = paras.0.lock().unwrap();
  let paras = paras_dict.get_mut(label).unwrap();

  let mut outline_paras_dict = outline_paras.0.lock().unwrap();
  let outline_paras = outline_paras_dict.get_mut(label).unwrap();

  let mut search_results_dict = search_results.0.lock().unwrap();
  let mut search_results = search_results_dict.get_mut(label).unwrap();
  // first unload file
  println!("unloading current file in window {:?}", label);
  paras.clear();
  outline_paras.clear();
  search_results.results.clear();
  search_results.last_query = None;
  search_results.para_texts.clear();
  // then load new one
  println!("loading {:?} file in window {:?}", path, label);
  let mut doc = Document::new();
  doc.load_file(&path);
  for para in doc.paras {
    let mut combined_text: String = "".to_string();
    for run in para.runs.iter() {
      combined_text.push_str(&run.text);
    }
    search_results.para_texts.push(combined_text);
    paras.push(para);
  }
  for outline_para in doc.outline_paras {
    outline_paras.push(outline_para);
  }
  println!("done loading file in window {:?}", label);
  true
}
#[tauri::command]
pub fn unload_file(
  paras: State<Paras>,
  outline_paras: State<OutlineParas>,
  search_results: State<SearchResults>,
  window: Window,
) -> bool {
  let label = window.label();
  let mut paras_dict = paras.0.lock().unwrap();
  let paras = paras_dict.get_mut(label).unwrap();

  let mut outline_paras_dict = outline_paras.0.lock().unwrap();
  let outline_paras = outline_paras_dict.get_mut(label).unwrap();

  let mut search_results_dict = search_results.0.lock().unwrap();
  let mut search_results = search_results_dict.get_mut(label).unwrap();
  paras.clear();
  outline_paras.clear();
  search_results.results.clear();
  search_results.last_query = None;
  search_results.para_texts.clear();
  return true;
}
#[tauri::command]
pub fn search(
  query: Query,
  i: usize,
  j: usize,
  paras: State<'_, Paras>,
  search_results: State<'_, SearchResults>,
  window: Window,
) -> Vec<SearchResult> {
  println!("searching with query: {:?}", query.text);
  let label = window.label();

  let mut paras_dict = paras.0.lock().unwrap();
  let paras = paras_dict.get_mut(label).unwrap();

  let mut search_results_dict = search_results.0.lock().unwrap();
  let mut search_results = search_results_dict.get_mut(label).unwrap();

  let query_text = match query.match_case {
    true => query.text.clone(),
    false => query.text.to_lowercase(),
  };
  // decide what to do with search_results.results
  if search_results.last_query.is_some() {
    let last_query = search_results.last_query.as_ref().unwrap();
    // if queries are the same, we can keep everything
    if &query == last_query {
      println!("reused old searches")
    } else
    // if last query is smaller version of this query, we can narrow down old search results first
    if query_text.contains(&last_query.text)
      && (query.match_case == last_query.match_case || query.match_case == true)
      && (query.only_outline == last_query.only_outline || query.only_outline == true)
    {
      println!("narrowed down old searches");
      // loop through search_results.results and remove all that are not in query
      // it is guaranteed that there will be less
      let mut new_results = Vec::new();
      for result in search_results.results.iter() {
        // if the amount of matches in text is larger than query_index
        let combined_text = search_results.para_texts[result.link].clone();
        if combined_text.to_lowercase().matches(&query_text).count() > result.query_index
          && (!query.only_outline || result.para.outline_level.is_some())
        {
          let mut new_result = result.clone();
          new_result.index = new_results.len();
          new_results.push(new_result);
        }
      }

      search_results.results = new_results;
    } else
    // if last query is bigger version of this query, we can clear old search results
    {
      println!("cleared old searches");
      search_results.results.clear();
    }
  } else {
    println!("no old searches")
  }
  search_results.last_query = Some(query.clone());
  // fill in the needed search_results.results
  let last_result = search_results.results.last();
  let mut l = 0;
  if last_result.is_some() {
    l = last_result.unwrap().para.index + 1;
  }
  while search_results.results.len() < j && l < paras.len() {
    if !query.only_outline || paras[l].outline_level.is_some() {
      let combined_text = &search_results.para_texts[l];
      let combined_text = match query.match_case {
        true => combined_text.clone(),
        false => combined_text.to_lowercase(),
      };
      for k in 0..combined_text.matches(&query_text).count() {
        let index = search_results.results.len();
        search_results.results.push(SearchResult {
          link: l.clone(),
          index: index,
          para: paras[l].clone(),
          query_index: k,
        });
      }
    }
    l += 1;
  }
  let mut result = Vec::new();
  println!("requested search results: {:?}..{:?}", i, j);
  if search_results.results.len() > 0 {
    for l in i..j {
      if (search_results.results.len() - 1) < l {
        break;
      }
      result.push(search_results.results[l].clone());
    }
  }
  println!("response length: {:?}", result.len());
  result
}
#[tauri::command]
pub fn clear_search(search_results: State<SearchResults>, window: Window) -> bool {
  let label = window.label();
  println!("unloading search");
  let mut search_results_dict = search_results.0.lock().unwrap();
  let mut search_results = search_results_dict.get_mut(label).unwrap();
  search_results.results.clear();
  search_results.last_query = None;
  return true;
}

#[tauri::command]
pub fn get_paras(i: usize, j: usize, paras: State<Paras>, window: Window) -> Vec<Para> {
  let label = window.label();
  let mut paras_dict = paras.0.lock().unwrap();
  let paras = paras_dict.get_mut(label).unwrap();
  let mut result = Vec::new();
  println!("requested paragraphs: {:?}..{:?}", i, j);
  if paras.len() > 0 {
    for l in i..j {
      if (paras.len() - 1) < l {
        break;
      }
      result.push(paras[l].clone());
    }
  }
  println!("response length: {:?}", result.len());
  return result;
}

#[tauri::command]
pub fn get_outline_paras(
  i: usize,
  j: usize,
  outline_paras: State<OutlineParas>,
  window: Window,
) -> Vec<OutlinePara> {
  let label = window.label();

  let mut outline_paras_dict = outline_paras.0.lock().unwrap();
  let outline_paras = outline_paras_dict.get_mut(label).unwrap();

  let mut result = Vec::new();
  println!("requested outline paragraphs: {:?}..{:?}", i, j);
  if outline_paras.len() > 0 {
    for l in i..j {
      if (outline_paras.len() - 1) < l {
        break;
      }
      result.push(outline_paras[l].clone());
    }
  }
  println!("response length: {:?}", result.len());
  return result;
}
#[tauri::command]
pub fn get_nearest_outline_para(
  para_index: usize,
  outline_paras: State<OutlineParas>,
  paras: State<Paras>,
  window: Window,
) -> Option<OutlinePara> {
  let label = window.label();

  let mut paras_dict = paras.0.lock().unwrap();
  let paras = paras_dict.get_mut(label).unwrap();

  let mut outline_paras_dict = outline_paras.0.lock().unwrap();
  let outline_paras = outline_paras_dict.get_mut(label).unwrap();
  // get the ratio position in paras as a starting point
  // so you minimize the amount you loop
  // theoretically you could still loop through all elements if outline elements are spaced unevenly
  // but this assumes normal data
  println!("finding nearest outline para to: {:?}", para_index);
  let start_index: usize = cmp::min(
    outline_paras.len() - 2,
    cmp::max(1, (para_index * outline_paras.len()) / paras.len()),
  );
  let start_outline_para = &outline_paras[start_index];
  let mut result: Option<OutlinePara> = None;
  println!(
    "starting at outline para: {:?}, with link: {:?}",
    start_index, start_outline_para.link
  );
  // find out which direction to go
  if start_outline_para.link >= para_index {
    println!("go backwards");
    // go backwards
    for i in (0..start_index).rev() {
      if outline_paras[i].link <= para_index {
        result = Some(outline_paras[i].clone());
        break;
      }
    }
  } else {
    println!("go forwards");
    for i in start_index..outline_paras.len() {
      if outline_paras[i].link > para_index && i > 1 {
        result = Some(outline_paras[i - 1].clone());
        break;
      } else if outline_paras[i].link == para_index {
        result = Some(outline_paras[i].clone());
        break;
      }
      // if nothing worked assume it was the last
      if outline_paras.last().is_some() {
        result = Some(outline_paras.last().unwrap().clone());
      }
    }
  }
  if result.is_some() {
    println!(
      "found outline_para: {:?}, with link: {:?}",
      result.clone().unwrap().index,
      result.clone().unwrap().link
    )
  } else {
    println!("didn't find outline_para");
  }
  result
}
