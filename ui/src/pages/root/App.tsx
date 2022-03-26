import type { Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';
import connectSocket from '../../network/websocket';

const App: Component = () => {
  connectSocket()
  return (
    <div class={styles.App}>
      <h1 className="max-w-2xl rounded overflow-hidden shadow-lg hover:shadow-xl p-4">
        
      </h1>
    </div>
  );
};

export default App;
