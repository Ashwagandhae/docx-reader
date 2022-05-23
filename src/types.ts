export type StyleType = {
  bold: boolean;
  underline: boolean;
  highlight: boolean;
  size: number;
  outline_level: number;
};
export type RunType = {
  text: string;
  style: StyleType;
};

export type ParaType = {
  runs: RunType[];
  outline_level: number;
  index?: number;
};
export type DocumentType = {
  paras: ParaType[];
};
export type OutlineItemType = {
  level: number;
  para: ParaType;
  index: number;
};
