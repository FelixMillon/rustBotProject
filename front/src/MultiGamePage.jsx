import React, { useState } from "react";
import GameInstance from "./GameInstance";

const MultiGamePage = () => {
    const [games, setGames] = useState([]);

    const addGameInstance = () => {
        setGames(prev => [...prev, Date.now()]);
    };

    return (
        <div style={{ padding: "20px", textAlign: "center" }}>
            <h1>Multi Game Manager</h1>
            <button onClick={addGameInstance} style={{ fontSize: "20px", margin: "20px" }}>
                ➕ Ajouter une partie
            </button>

            <div
                style={{
                    display: "flex",
                    flexDirection: "row",
                    gap: "20px",
                    overflowX: "auto",
                    padding: "20px",
                    borderTop: "2px solid #ccc",
                }}
            >
                {games.map((id, index) => (
                    <div
                        key={id}
                        style={{
                            borderLeft: index !== 0 ? "2px solid #ccc" : "none",
                            paddingLeft: index !== 0 ? "20px" : "0",
                        }}
                    >
                        <GameInstance />
                    </div>
                ))}
            </div>
        </div>
    );
};

export default MultiGamePage;
