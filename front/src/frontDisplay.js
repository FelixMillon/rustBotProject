import React, { useEffect, useState } from "react";
import axios from "axios";
import "./App.css";

const FrontDisplay = () => {
    const [gameState, setGameState] = useState({
        map: [],
        crystal_count: 0,
        energy_count: 0,
    });

    const [isPaused, setIsPaused] = useState(false);

    const fetchGameState = async () => {
        try {
            const response = await axios.get("http://127.0.0.1:3001/state");
            setGameState(response.data);
        } catch (error) {
            console.error("Error fetching game state:", error);
        }
    };

    const resetGame = async () => {
        try {
            await axios.get("http://127.0.0.1:3001/reset");
            await fetchGameState(); // Rafraîchir après reset
        } catch (error) {
            console.error("Error resetting game:", error);
        }
    };

    useEffect(() => {
        const intervalId = setInterval(() => {
            if (!isPaused) {
                fetchGameState();
            }
        }, 200);

        return () => clearInterval(intervalId);
    }, [isPaused]);

    const togglePause = () => {
        setIsPaused((prev) => !prev);
    };

    const getCellClass = (cell) => {
        switch (cell) {
            case "C":
                return "cell crystal";
            case "E":
                return "cell energy";
            case "S":
                return "cell scout";
            case "G":
                return "cell gatherer";
            default:
                return "cell";
        }
    };

    const getCellIcon = (cell) => {
        switch (cell) {
            case "8":
                return "🌳";
            case "C":
                return "💎";
            case "E":
                return "⚡️";
            case "S":
                return "🤖";
            case "G":
                return "👷";
            case "#":
                return "🏰";
            default:
                return "";
        }
    };

    const flatMap = gameState.map.flat();

    return (
        <div style={{ textAlign: "center" }}>
            <h1>Game Map</h1>
            <div
                className="map-grid"
                style={{
                    display: "grid",
                    gridTemplateColumns: `repeat(${gameState.map[0]?.length || 1}, 20px)`,
                    justifyContent: "center",
                    margin: "auto",
                }}
            >
                {flatMap.map((cell, index) => (
                    <div
                        key={index}
                        className={getCellClass(cell)}
                        style={{
                            width: "20px",
                            height: "20px",
                            display: "flex",
                            alignItems: "center",
                            justifyContent: "center",
                            fontSize: "14px",
                        }}
                    >
                        {getCellIcon(cell)}
                    </div>
                ))}
            </div>

            <div style={{ marginTop: "20px" }}>
                <p>Crystal: {gameState.crystal_count}</p>
                <p>Energy: {gameState.energy_count}</p>
                <button onClick={togglePause} style={{ marginRight: "10px" }}>
                    {isPaused ? "Start" : "Pause"}
                </button>
                <button onClick={resetGame}>Reset</button>
            </div>
        </div>
    );
};

export default FrontDisplay;
