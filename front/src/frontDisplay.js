import React, { useEffect, useState } from "react";
import axios from "axios";
import "./App.css";

const FrontDisplay = () => {
    const [gameState, setGameState] = useState({
        map: [],
        crystal_count: 0,
        energy_count: 0,
    });
    const [speed, setSpeed] = useState(500);
    const [isPaused, setIsPaused] = useState(false);
    const [showResetPopup, setShowResetPopup] = useState(false);

    const [resetValues, setResetValues] = useState({
        columns: 15,
        rows: 15,
        seed: 40,
        gatherers: 3,
        scouts: 7,
        resources: 15,
        empty_display: ' ',
        obstacle_display: '8',
        base_display: '#',
        scout_display: 'S',
        gatherer_display: 'G'
    });

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
            await axios.post("http://127.0.0.1:3001/reset", resetValues);
            await fetchGameState();
            setShowResetPopup(false);
        } catch (error) {
            console.error("Error resetting game:", error);
        }
    };

    const startGame = async () => {
        try {
            await axios.post("http://127.0.0.1:3001/start", resetValues);
            await fetchGameState();
            setShowResetPopup(false);
        } catch (error) {
            console.error("Error starting game:", error);
        }
    };

    const stopGame = async () => {
        try {
            await axios.post("http://127.0.0.1:3001/stop");
            await fetchGameState();
        } catch (error) {
            console.error("Error stopping game:", error);
        }
    };
    
    useEffect(() => {
        const intervalId = setInterval(() => {
            if (!isPaused) {
                fetchGameState();
            }
        }, speed);

        return () => clearInterval(intervalId);
    }, [isPaused, speed]);

    const togglePause = () => {
        setIsPaused(prev => !prev);
    };

    const increaseSpeed = () => {
        setSpeed(prev => Math.max(100, prev - 100));
    };

    const decreaseSpeed = () => {
        setSpeed(prev => Math.min(1000, prev + 100));
    };

    const getCellClass = (cell) => {
        switch (cell) {
            case "C": return "cell crystal";
            case "E": return "cell energy";
            case "S": return "cell scout";
            case "G": return "cell gatherer";
            default: return "cell";
        }
    };

    const getCellIcon = (cell) => {
        switch (cell) {
            case "8": return "🌳";
            case "C": return "💎";
            case "E": return "⚡️";
            case "S": return "🤖";
            case "G": return "🧑‍🌾";
            case "#": return "🏰";
            default: return "";
        }
    };

    const flatMap = gameState.map.flat();

    const handleResetChange = (e) => {
        const { name, value } = e.target;
        let val = parseInt(value);;

        setResetValues(prev => ({
            ...prev,
            [name]: val,
        }));
    };

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

                {/* Bouton Reset */}
                <button onClick={() => setShowResetPopup(true)} style={{ marginLeft: "10px" }}>
                    {gameState.map.length === 0 ? "🚀 Start" : "🔄 Reset"}
                </button>
                {/* Bouton Stop */}
                {gameState.map.length > 0 && (
                    <button onClick={stopGame} style={{ marginLeft: "10px" }}>
                        🛑 Finish
                    </button>
                )}
            </div>

            <div style={{ marginTop: "20px", display: "flex", justifyContent: "center", alignItems: "center" }}>
                <button onClick={decreaseSpeed} style={{ fontSize: "20px", marginLeft: "10px" }}>⏪</button>
                <button onClick={togglePause} style={{ margin: "0 10px", fontSize: "20px" }}>
                    {isPaused ? "▶️" : "⏸️"}
                </button>
                <button onClick={increaseSpeed} style={{ fontSize: "20px", marginRight: "10px" }}>⏩</button>
            </div>

            {/* ===================== POPUP ===================== */}
            {showResetPopup && (
                <div
                    style={{
                        position: "fixed",
                        top: 0,
                        left: 0,
                        width: "100vw",
                        height: "100vh",
                        backgroundColor: "rgba(0, 0, 0, 0.5)",
                        display: "flex",
                        alignItems: "center",
                        justifyContent: "center",
                    }}
                >
                    <div
                        style={{
                            backgroundColor: "rgba(32, 32, 32, 0.5)",
                            padding: "30px",
                            borderRadius: "10px",
                            minWidth: "300px",
                            textAlign: "left",
                        }}
                    >
                        <h2>{gameState.map.length === 0 ? "Paramètres de Démarrage" : "Paramètres de Reset"}</h2>
                        <div>
                            <label>
                                Columns (15–100): 
                                <input type="string" name="columns" value={resetValues.columns} onChange={handleResetChange} />
                            </label>
                        </div>
                        <div>
                            <label>
                                Rows (15–100): 
                                <input type="number" name="rows" value={resetValues.rows} onChange={handleResetChange} />
                            </label>
                        </div>
                        <div>
                            <label>
                                Gatherers (0–15): 
                                <input type="number" name="gatherers" value={resetValues.gatherers} onChange={handleResetChange} />
                            </label>
                        </div>
                        <div>
                            <label>
                                Scouts (1–15): 
                                <input type="number" name="scouts" value={resetValues.scouts} onChange={handleResetChange} />
                            </label>
                        </div>
                        <div>
                            <label>
                                Ressources (1-50): 
                                <input type="number" name="resources" value={resetValues.resources} onChange={handleResetChange} />
                            </label>
                        </div>
                        <div>
                            <label>
                                Graine: 
                                <input type="number" name="seed" value={resetValues.seed} onChange={handleResetChange} />
                            </label>
                        </div>

                        <div style={{ marginTop: "20px", display: "flex", justifyContent: "space-between" }}>
                            <button onClick={gameState.map.length === 0 ? startGame : resetGame}>
                                ✅ Valider
                            </button>
                            <button onClick={() => setShowResetPopup(false)}>❌ Annuler</button>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};

export default FrontDisplay;
