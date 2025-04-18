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
    const [resetValues, setResetValues] = useState({
        columns: 15,
        rows: 15,
        seed: 40,
        gatherers: 3,
        scouts: 7,
        empty_display: ' ',
        obstacle_display: '8',
        base_display: '#',
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
        } catch (error) {
            console.error("Error resetting game:", error);
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
            case "G": return "👷";
            case "#": return "🏰";
            default: return "";
        }
    };

    const flatMap = gameState.map.flat();

    const handleResetChange = (e) => {
        const { name, value } = e.target;
        let val = value;

        if (name === 'columns' || name === 'rows') {
            val = Math.max(15, Math.min(100, parseInt(value)));
        } else if (name === 'gatherers') {
            val = Math.max(0, Math.min(5, parseInt(value)));
        } else if (name === 'scouts') {
            val = Math.max(1, Math.min(15, parseInt(value)));
        } else if (name === 'seed') {
            val = parseInt(value);
        }

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
                <button onClick={resetGame} style={{ marginLeft: "10px" }}>
                    🔄 Reset
                </button>
            </div>

            <div style={{ marginTop: "20px", display: "flex", justifyContent: "center", alignItems: "center" }}>
                <button onClick={decreaseSpeed} style={{ fontSize: "20px", marginLeft: "10px" }}>⏪</button>
                {/* Boutons Pause/Start */}
                <button onClick={togglePause} style={{ margin: "0 10px", fontSize: "20px" }}>
                    {isPaused ? "▶️" : "⏸️"}
                </button>
                <button onClick={increaseSpeed} style={{ fontSize: "20px", marginRight: "10px" }}>⏩</button>
                
            </div>

            <div style={{ marginTop: "30px" }}>
                <h2>Reset Game</h2>
                <div>
                    <label>
                        Columns (min 15 max 100):
                        <input
                            type="number"
                            name="columns"
                            value={resetValues.columns}
                            onChange={handleResetChange}
                            min="15"
                            max="100"
                        />
                    </label>
                </div>
                <div>
                    <label>
                        Rows (min 15 max 100):
                        <input
                            type="number"
                            name="rows"
                            value={resetValues.rows}
                            onChange={handleResetChange}
                            min="15"
                            max="100"
                        />
                    </label>
                </div>
                <div>
                    <label>
                        Gatherers (0-5):
                        <input
                            type="number"
                            name="gatherers"
                            value={resetValues.gatherers}
                            onChange={handleResetChange}
                            min="0"
                            max="5"
                        />
                    </label>
                </div>
                <div>
                    <label>
                        Scouts (1-15):
                        <input
                            type="number"
                            name="scouts"
                            value={resetValues.scouts}
                            onChange={handleResetChange}
                            min="1"
                            max="15"
                        />
                    </label>
                </div>
                <div>
                    <label>
                        Graine:
                        <input
                            type="number"
                            name="seed"
                            value={resetValues.seed}
                            onChange={handleResetChange}
                        />
                    </label>
                </div>
            </div>
        </div>
    );
};

export default FrontDisplay;
