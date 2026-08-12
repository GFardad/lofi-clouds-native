import { useEffect, useRef, useState } from "react";

const RADIO_URL = "https://radio.lofi-clouds.com/#/";

export default function RadioPlayer() {
  const webviewRef = useRef<HTMLIFrameElement | null>(null);
  const [status, setStatus] = useState<string>("loading");

  useEffect(() => {
    const frame = webviewRef.current;
    if (!frame) return;

    frame.src = RADIO_URL;

    const onLoad = () => setStatus("ready");
    const onError = () => setStatus("error");

    frame.addEventListener("load", onLoad);
    frame.addEventListener("error", onError);

    return () => {
      frame.removeEventListener("load", onLoad);
      frame.removeEventListener("error", onError);
    };
  }, []);

  const post = (action: string) => {
    const frame = webviewRef.current;
    if (!frame || !frame.contentWindow) return;
    frame.contentWindow.postMessage({ type: "radio-command", action }, "*");
  };

  return (
    <div className="player">
      <header className="titlebar">
        <div className="title">
          <span className="logo" aria-hidden>☁️</span>
          <span className="label">Lofi Clouds Radio</span>
        </div>
        <div className="status">{status === "ready" ? "● Live" : status === "error" ? "○ Error" : "… Loading"}</div>
        <div className="transports">
          <button onClick={() => post("play")} className="transport" title="Play">▶</button>
          <button onClick={() => post("pause")} className="transport" title="Pause">⏸</button>
          <button onClick={() => post("next")} className="transport" title="Next">⏭</button>
        </div>
      </header>
      <div className="frame-wrap">
        <iframe
          ref={webviewRef}
          title="Lofi Clouds Radio"
          src={RADIO_URL}
          allow="autoplay; encrypted-media"
          sandbox="allow-scripts allow-same-origin allow-presentation"
        />
      </div>
    </div>
  );
}
