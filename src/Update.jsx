import { useState, useEffect } from "react";
import { check } from "@tauri-apps/plugin-updater";

export function UpdateModal() {
  const [update, setUpdate] = useState(null);
  const [downloading, setDownloading] = useState(false);
  const [progress, setProgress] = useState(0);
  const [statusText, setStatusText] = useState("");

  useEffect(() => {
    checkForUpdates();
  }, []);

  async function checkForUpdates() {
    try {
      const res = await check();
      if (res) {
        setUpdate(res);
      }
    } catch (err) {}
  }

  async function handleInstall() {
    if (!update) return;
    setDownloading(true);
    setStatusText("Starting download...");

    let downloadedBytes = 0;
    let totalBytes = 0;

    try {
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            totalBytes = event.data.contentLength || 0;
            setStatusText("Downloading update...");
            break;

          case "Progress":
            downloadedBytes += event.data.chunkLength;
            if (totalBytes > 0) {
              const percentage = Math.round(
                (downloadedBytes / totalBytes) * 100,
              );
              setProgress(percentage);
              setStatusText(`Downloading: ${percentage}%`);
            } else {
              setStatusText(`${(downloadedBytes / 1024 / 1024).toFixed(1)} MB`);
            }
            break;

          case "Finished":
            setStatusText("Installing update...");
            break;
        }
      });

      setStatusText("Restarting...");
    } catch (err) {
      setStatusText("There was an error updating");
      setDownloading(false);
    }
  }

  if (!update) return null;

  return (
    <div style={styles.overlay}>
      <div style={styles.card}>
        <h3 style={styles.title}>Update available</h3>
        <p style={styles.version}>Version {update.version}</p>

        {update.body && <p style={styles.notes}>{update.body}</p>}

        {downloading ? (
          <div style={styles.progressContainer}>
            <div style={styles.progressBarTrack}>
              <div
                style={{ ...styles.progressBarFill, width: `${progress}%` }}
              />
            </div>
            <span style={styles.statusText}>{statusText}</span>
          </div>
        ) : (
          <div style={styles.actions}>
            <button style={styles.btnSecondary} onClick={() => setUpdate(null)}>
              Not now
            </button>
            <button style={styles.btnPrimary} onClick={handleInstall}>
              Update
            </button>
          </div>
        )}
      </div>
    </div>
  );
}

const styles = {
  overlay: {
    position: "fixed",
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    backgroundColor: "rgba(0, 0, 0, 0.85)",
    backdropFilter: "blur(4px)",
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    zIndex: 9999,
    fontFamily: "Georgia, serif",
  },
  card: {
    backgroundColor: "#121212",
    border: "1px solid #333",
    padding: "24px 32px",
    width: "380px",
    boxShadow: "0 8px 32px rgba(0,0,0,0.9)",
    color: "#e0e0e0",
    textAlign: "center",
  },
  title: {
    margin: "0 0 8px 0",
    fontSize: "1.2rem",
    fontWeight: "normal",
    color: "#fff",
    letterSpacing: "0.5px",
  },
  version: {
    margin: "0 0 16px 0",
    fontSize: "0.9rem",
    color: "#888",
  },
  notes: {
    fontSize: "0.85rem",
    color: "#aaa",
    marginBottom: "20px",
    textAlign: "left",
    maxHeight: "80px",
    overflowY: "auto",
  },
  progressContainer: {
    marginTop: "16px",
  },
  progressBarTrack: {
    width: "100%",
    height: "6px",
    backgroundColor: "#222",
    border: "1px solid #444",
    overflow: "hidden",
    marginBottom: "8px",
  },
  progressBarFill: {
    height: "100%",
    backgroundColor: "#c5a059",
    transition: "width 0.2s ease-in-out",
  },
  statusText: {
    fontSize: "0.8rem",
    color: "#aaa",
  },
  actions: {
    display: "flex",
    justifyContent: "flex-end",
    gap: "12px",
    marginTop: "20px",
  },
  btnPrimary: {
    backgroundColor: "transparent",
    border: "1px solid #c5a059",
    color: "#c5a059",
    padding: "6px 16px",
    cursor: "pointer",
    fontFamily: "inherit",
    fontSize: "0.9rem",
    transition: "all 0.2s",
  },
  btnSecondary: {
    backgroundColor: "transparent",
    border: "1px solid #444",
    color: "#888",
    padding: "6px 16px",
    cursor: "pointer",
    fontFamily: "inherit",
    fontSize: "0.9rem",
  },
};
