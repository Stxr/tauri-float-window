import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [isAlwaysOnTop, setIsAlwaysOnTop] = useState(false);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  async function toggleAlwaysOnTop() {
    const newAlwaysOnTopState = !isAlwaysOnTop;
    await invoke("set_always_on_top", { alwaysOnTop: newAlwaysOnTopState });
    setIsAlwaysOnTop(newAlwaysOnTopState);
  }

  async function minimizeWindow() {
    await invoke("minimize_window");
  }

  async function toggleMaximizeWindow() {
    await invoke("toggle_maximize_window");
  }

  async function closeWindow() {
    await invoke("close_window");
  }

  useEffect(() => {
    const minimizeBtn = document.getElementById('titlebar-minimize');
    const maximizeBtn = document.getElementById('titlebar-maximize');
    const closeBtn = document.getElementById('titlebar-close');
    const titlebar = document.querySelector('[data-tauri-drag-region]') as HTMLElement;

    minimizeBtn?.addEventListener('click', minimizeWindow);
    maximizeBtn?.addEventListener('click', toggleMaximizeWindow);
    closeBtn?.addEventListener('click', closeWindow);

    // Double click to maximize - CSS handles the dragging automatically
    titlebar?.addEventListener('dblclick', toggleMaximizeWindow);

    return () => {
      minimizeBtn?.removeEventListener('click', minimizeWindow);
      maximizeBtn?.removeEventListener('click', toggleMaximizeWindow);
      closeBtn?.removeEventListener('click', closeWindow);
      titlebar?.removeEventListener('dblclick', toggleMaximizeWindow);
    };
  }, []);

  return (
    <div className="app-container">
      <div data-tauri-drag-region className="titlebar">
        <div className="traffic-lights">
          <button id="titlebar-close" title="close" className="traffic-light close">
            <svg width="10" height="10" viewBox="0 0 10 10">
              <path d="M3 3l4 4M3 7l4-4" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
            </svg>
          </button>
          <button id="titlebar-minimize" title="minimize" className="traffic-light minimize">
            <svg width="10" height="10" viewBox="0 0 10 10">
              <path d="M2 5h6" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
            </svg>
          </button>
          <button id="titlebar-maximize" title="maximize" className="traffic-light maximize">
            <svg width="10" height="10" viewBox="0 0 10 10">
              <path d="M2 2h6v6H2z" stroke="currentColor" strokeWidth="1.5" fill="none" />
            </svg>
          </button>
        </div>
        <div className="titlebar-title">htool mirror</div>
        <div className="titlebar-controls">

          <button
            id="titlebar-pin"
            title={isAlwaysOnTop ? "Unpin Window" : "Pin Window"}
            className={`titlebar-button pin ${isAlwaysOnTop ? 'active' : ''}`}
            onClick={toggleAlwaysOnTop}
          >
            <svg width="14" height="14" viewBox="0 0 14 14">
              {isAlwaysOnTop ? (
                <path d="M7 1v12M1 7h12" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
              ) : (
                <path d="M7 1v6M4 4l3 3 3-3" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
              )}
            </svg>
          </button>
        </div>
      </div>
      <main>

        <h1>Welcome to Tauri + React111</h1>

        <div className="row">
          <a href="https://vitejs.dev" target="_blank">
            <img src="/vite.svg" className="logo vite" alt="Vite logo" />
          </a>
          <a href="https://tauri.app" target="_blank">
            <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
          </a>
          <a href="https://reactjs.org" target="_blank">
            <img src={reactLogo} className="logo react" alt="React logo" />
          </a>
        </div>
        <p>Click on the Tauri, Vite, and React logos to learn more.</p>

        <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="submit">Greet</button>
        </form>
        <p>{greetMsg}</p>
      </main>
    </div>
  );
}

export default App;
