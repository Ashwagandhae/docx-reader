class Messenger {
  callbacks: { [key: string]: ((...args: any[]) => void)[] } = {};
  constructor() {
    this.callbacks = {};
  }
  on(event: string, callback: (...args: any[]) => void) {
    if (!this.callbacks[event]) {
      this.callbacks[event] = [];
    }
    this.callbacks[event].push(callback);
  }
  emit(event: string, ...args: any[]) {
    if (!this.callbacks[event]) {
      return;
    }
    this.callbacks[event].forEach((callback) => {
      callback(...args);
    });
  }
}
export let messenger = new Messenger();
