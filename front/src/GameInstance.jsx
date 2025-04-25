import React, { useEffect, useRef , useState, forwardRef, useImperativeHandle } from "react";
import { Canvas, useFrame  } from "@react-three/fiber";
import { OrbitControls, Stats, useGLTF } from "@react-three/drei";
import axios from "axios";
import "./App.css";

const GameInstance = forwardRef((props, ref) => {
    useImperativeHandle(ref, () => ({
        stopGame
    }));
    const [gameState, setGameState] = useState({
        map: [],
        crystal_count: 0,
        energy_count: 0,
    });
    const [speed, setSpeed] = useState(500);
    const [isPaused, setIsPaused] = useState(false);
    const [showResetPopup, setShowResetPopup] = useState(false);
    const [gameId, setGameId] = useState(null);

    const Base = ({ position }) => {
        return (
            <group position={position}>
                {/* Corps de la maison */}
                <mesh position={[0, 0.75, 0]}>
                    <boxGeometry args={[2, 1.4, 2]} />
                    <meshStandardMaterial color="#b5651d" />
                </mesh>
    
                {/* Toit */}
                <mesh position={[0, 1.85, 0]}  rotation={[0, Math.PI / 4, 0]}>
                    <coneGeometry args={[1.6, 1, 4]} />
                    <meshStandardMaterial color="#8b0000" />
                </mesh>

            </group>
        );
    };
    const Tree = ({ position }) => {
        return (
            <group position={position}>
                {/* Tronc */}
                <mesh position={[0, 0.5, 0]}>
                    <cylinderGeometry args={[0.2, 0.2, 1, 16]} />
                    <meshStandardMaterial color="#8B4513" />
                </mesh>
    
                {/* Feuillage */}
                <mesh position={[0, 1.3, 0]}>
                    <sphereGeometry args={[0.6, 32, 32]} />
                    <meshStandardMaterial color="green" />
                </mesh>
            </group>
        );
    };

    const Robot = ({ position, color = "#cccccc" }) => {
        return (
            <group position={position}>

                <mesh position={[0, 0.625, 0]}>
                    <boxGeometry args={[0.5, 0.75, 0.3]} />
                    <meshStandardMaterial color={color} />
                </mesh>
        
                <mesh position={[0, 1.15, 0]}>
                    <boxGeometry args={[0.4, 0.4, 0.4]} />
                    <meshStandardMaterial color={color} />
                </mesh>
        
                <mesh position={[-0.4, 0.625, 0]}>
                    <cylinderGeometry args={[0.05, 0.05, 0.5, 16]} />
                    <meshStandardMaterial color="#666" />
                </mesh>
        
                <mesh position={[0.4, 0.625, 0]}>
                    <cylinderGeometry args={[0.05, 0.05, 0.5, 16]} />
                    <meshStandardMaterial color="#666" />
                </mesh>
        
                <mesh position={[-0.15, 0.125, 0]}>
                    <cylinderGeometry args={[0.075, 0.075, 0.25, 16]} />
                    <meshStandardMaterial color="#333" />
                </mesh>
        
                <mesh position={[0.15, 0.125, 0]}>
                    <cylinderGeometry args={[0.075, 0.075, 0.25, 16]} />
                    <meshStandardMaterial color="#333" />
                </mesh>
            </group>
        );
    };

    const Crystal = ({ position, color = "cyan" }) => {
        return (
            <group position={position}>
                {/* Cristal de base */}
                <mesh position={[0, 1, 0]}>
                    <coneGeometry args={[0.6, 1.5, 4]} />
                    <meshStandardMaterial color={color} roughness={0.2} metalness={0.5} transparent={true} opacity={0.8} />
                </mesh>
                
                {/* Cône 1 - Direction 1 */}
                <mesh position={[0, 1, 0.6]} rotation={[Math.PI / 4, Math.PI / 4, 0]}>
                    <coneGeometry args={[0.4, 1.5, 4]} />
                    <meshStandardMaterial color={color} roughness={0.2} metalness={0.5} transparent={true} opacity={0.8} />
                </mesh>
                {/* Cône 2 - Direction 2 */}
                <mesh position={[0, 1, -0.5]} rotation={[-Math.PI / 5, Math.PI / 4, 0]}>
                    <coneGeometry args={[0.3, 1, 4]} />
                    <meshStandardMaterial color={color} roughness={0.2} metalness={0.5} transparent={true} opacity={0.8} />
                </mesh>
                {/* Cône 2 - Direction 2 */}
                <mesh position={[0, 1, 0.3]} rotation={[Math.PI / 5, Math.PI / 4, 0]}>
                    <coneGeometry args={[0.2, 1, 4]} />
                    <meshStandardMaterial color={color} roughness={0.2} metalness={0.5} transparent={true} opacity={0.8} />
                </mesh>
            </group>
        );
    };
    const Lightning = ({ position, color = "yellow" }) => {
        return (
            <group position={position}>
                {/* Premier segment du haut */}
                <mesh position={[0, 2.7, 0]} rotation={[Math.PI / 4, 0, 0]}>
                    <cylinderGeometry args={[0.05, 0.1, 1, 8]} />
                    <meshStandardMaterial color={color} emissive={color} emissiveIntensity={0.8} />
                </mesh>
    
                {/* Deuxième segment, un peu plus bas */}
                <mesh position={[0, 1.9, 0]} rotation={[-Math.PI / 6, 0, 0]}>
                    <cylinderGeometry args={[0.05, 0.1, 1, 8]} />
                    <meshStandardMaterial color={color} emissive={color} emissiveIntensity={0.8} />
                </mesh>
    
                {/* Troisième segment */}
                <mesh position={[0, 1.1, 0]} rotation={[Math.PI / 6, 0, 0]}>
                    <cylinderGeometry args={[0.05, 0.1, 1, 8]} />
                    <meshStandardMaterial color={color} emissive={color} emissiveIntensity={0.8} />
                </mesh>
    
                {/* Quatrième segment, presque au bas */}
                <mesh position={[0, 0.3, 0]} rotation={[-Math.PI / 4, 0, 0]}>
                    <cylinderGeometry args={[0.05, 0.1, 1, 8]} />
                    <meshStandardMaterial color={color} emissive={color} emissiveIntensity={0.8} />
                </mesh>
            </group>
        );
    };


    const EnergyCore = ({ position = [0, 0, 0], color = "yellow" }) => {
        const meshRef = useRef();
    
        useFrame(() => {
            if (meshRef.current) {
                meshRef.current.rotation.y += 0.01;
                meshRef.current.rotation.x += 0.005;
            }
        });
    
        return (
            <mesh position={position} ref={meshRef}>
                <icosahedronGeometry args={[0.7, 0]} />
                <meshStandardMaterial 
                    color={color}
                    emissive={color}
                    emissiveIntensity={1.8}
                    metalness={0.8}
                    roughness={0.2}
                />
            </mesh>
        );
    };
    const CellMesh = ({ x, y, type }) => {
        let color = "#333";
    
        switch (type) {
            case "8": return <Tree position={[x, 0, y]} />;
            case "C": return <Crystal position={[x, -0.4, y]}/>;
            case "E": return <EnergyCore position={[x, 1, y]}/>;
            case "S": return <Robot position={[x, 0, y]} color="lime" />;
            case "G": return <Robot position={[x, 0, y]} color="purple" />;
            case "#": return <Base position={[x, 0, y]}/>;
        }
    };
    const [resetValues, setResetValues] = useState({
        columns: 25,
        rows: 25,
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

    const handleResetChange = (e) => {
        const { name, value } = e.target;
        let val = parseInt(value);
        setResetValues(prev => ({ ...prev, [name]: val }));
    };

    return (
        <div className="game-instance">
            <h3>Instance de jeu</h3>
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
            <p>💎 Crystal: {gameState.crystal_count} ⚡️ Energy: {gameState.energy_count}</p>
            <div className="map-container">
                <Canvas camera={{ position: [30, 20, 30], fov: 100 }}>
                    <ambientLight intensity={0.5} />
                    <directionalLight position={[10, 10, 5]} intensity={1} />
                    <OrbitControls />

                    {/* Sol */}
                    <mesh receiveShadow position={[resetValues.columns / 2, -0.25, resetValues.rows / 2]}>
                        <boxGeometry args={[resetValues.columns, 0.5, resetValues.rows]} />
                        <meshStandardMaterial color="#388E3C" />
                    </mesh>

                    {/* Grille 3D */}
                    {gameState.map.map((row, y) =>
                        row.map((cell, x) => (
                            <CellMesh key={`${x}-${y}`} x={x} y={y} type={cell} />
                        ))
                    )}
                </Canvas>
            </div>
        </div>
    );
});

export default GameInstance;
