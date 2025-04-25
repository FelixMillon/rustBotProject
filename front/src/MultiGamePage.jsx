import React, { useState } from "react";
import GameInstance from "./GameInstance";

const MultiGamePage = () => {
    const [games, setGames] = useState([]);

    const addGameInstance = () => {
        const newGame = {
            key: Date.now(),
            ref: React.createRef()
        };
        setGames(prev => [...prev, newGame]);
    };

    const removeGameInstance = async (key) => {
        const game = games.find(g => g.key === key);
        if (game && game.ref.current) {
            await game.ref.current.stopGame();
        }
        setGames(prev => prev.filter(g => g.key !== key));
    };

    return (
        <div className="page-container">
            <h1 className="page-title" >Multi Game Manager</h1>
            <button 
                onClick={addGameInstance} 
                className="add-game-button"
            >
                ➕ Ajouter une partie
            </button>

            <div className="games-wrapper">
                {games.map((game, index) => {
                    const isSingle = games.length === 1;
                    const width = isSingle ? "100%" : `${100 / games.length}%`;

                    return (
                        <div 
                            key={game.key}
                            className={`game-column ${index !== 0 ? "with-border" : ""}`}
                            style={{ width }}
                        >
                            <button
                                onClick={() => removeGameInstance(game.key)}
                                title="Fermer cette instance"
                                className="close-button"
                                onMouseEnter={(e) => e.currentTarget.style.transform = "scale(1.1)"}
                                onMouseLeave={(e) => e.currentTarget.style.transform = "scale(1)"}
                            >
                                <span>✖</span>
                            </button>
                            <GameInstance ref={game.ref} />
                        </div>
                    );
                })}
            </div>
        </div>
    );
};

export default MultiGamePage;
