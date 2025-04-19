import React, { useEffect, useState } from "react";
import axios from "axios";
import "./App.css";

const GameInstance = () => {
    const [gameState, setGameState] = useState({
        map: [],
        crystal_count: 0,
        energy_count: 0,
    });
    const [speed, setSpeed] = useState(500);
    const [isPaused, setIsPaused] = useState(false);
    const [showResetPopup, setShowResetPopup] = useState(false);
    const [gameId, setGameId] = useState(null);

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

    const fetchGameState = async (id = gameId) => {
        if (!id) return;
        try {
            const response = await axios.get(`http://127.0.0.1:3001/state/${id}`);
            setGameState(response.data);
        } catch (error) {
            console.error("Error fetching game state:", error);
        }
    };

    const resetGame = async () => {
        if (!gameId) return;
        try {
            await axios.post(`http://127.0.0.1:3001/reset/${gameId}`, resetValues);
            await fetchGameState();
            setShowResetPopup(false);
        } catch (error) {
            console.error("Error resetting game:", error);
        }
    };

    const startGame = async () => {
        try {
            const response = await axios.post("http://127.0.0.1:3001/start", resetValues);
            const id = response.data;
            setGameId(id);
            await fetchGameState(id);
            setShowResetPopup(false);
        } catch (error) {
            console.error("Error starting game:", error);
        }
    };

    const stopGame = async () => {
        if (!gameId) return;
        try {
            await axios.post(`http://127.0.0.1:3001/stop/${gameId}`);
            setGameId(null);
            setGameState({
                map: [],
                crystal_count: 0,
                energy_count: 0,
            });
        } catch (error) {
            console.error("Error stopping game:", error);
        }
    };

    useEffect(() => {
        const intervalId = setInterval(() => {
            if (!isPaused && gameId) {
                fetchGameState();
            }
        }, speed);
        return () => clearInterval(intervalId);
    }, [isPaused, speed, gameId]);

    const togglePause = () => setIsPaused(prev => !prev);
    const increaseSpeed = () => setSpeed(prev => Math.max(100, prev - 100));
    const decreaseSpeed = () => setSpeed(prev => Math.min(1000, prev + 100));

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
        let val = parseInt(value);
        setResetValues(prev => ({ ...prev, [name]: val }));
    };

    return (
        <div className="game-instance">
            <h3>Instance de jeu</h3>
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

            <p>💎 Crystal: {gameState.crystal_count}</p>
            <p>⚡️ Energy: {gameState.energy_count}</p>

            <div>
                <button onClick={() => setShowResetPopup(true)}>
                    {gameState.map.length === 0 ? "🚀 Start" : "🔄 Reset"}
                </button>
                {gameState.map.length > 0 && (
                    <button onClick={stopGame}>🛑 Finish</button>
                )}
            </div>

            <div>
                <button onClick={decreaseSpeed}>⏪</button>
                <button onClick={togglePause}>{isPaused ? "▶️" : "⏸️"}</button>
                <button onClick={increaseSpeed}>⏩</button>
            </div>

            {showResetPopup && (
                <div className="popup-backdrop">
                    <div className="popup">
                        <h4>Paramètres</h4>
                        {["columns", "rows", "gatherers", "scouts", "resources", "seed"].map((key) => (
                            <div key={key}>
                                <label>{key}: <input type="number" name={key} value={resetValues[key]} onChange={handleResetChange} /></label>
                            </div>
                        ))}
                        <div>
                            <button onClick={gameState.map.length === 0 ? startGame : resetGame}>✅ Valider</button>
                            <button onClick={() => setShowResetPopup(false)}>❌ Annuler</button>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};

export default GameInstance;
