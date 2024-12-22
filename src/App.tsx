import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import "./App.css";
import { useInterval } from "react-use";

function App() {
  const [isMuted, setMuted] = useState(false);

  useInterval(() => {
    (async () => {
      setMuted((await invoke("get_mic_status")) === "true");
    })();
  }, 2000);

  return (
    <main className="container">
      <div className="mute_status_bar">
        <div className={`mute_status_animation ${isMuted ? "muted" : ""}`} />
      </div>
    </main>
  );
}

export default App;
