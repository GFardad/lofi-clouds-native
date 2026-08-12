import { useEffect, useState } from "react";
import RadioPlayer from "./RadioPlayer";

export default function App() {
  const [view, setView] = useState<"player" | "about">("player");

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "1") setView("player");
      if (event.key === "2") setView("about");
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, []);

  return (
    <div className="app">
      {view === "player" && <RadioPlayer />}
      {view === "about" && (
        <div className="about">
          <h1>Lofi Clouds Radio</h1>
          <p>Native wrapper for https://radio.lofi-clouds.com/#/</p>
          <p className="hint">Press 1 to open player, 2 for about.</p>
        </div>
      )}
    </div>
  );
}
