import "./App.css";
import { aFunction } from "./dir1/mod1";

function App() {
    return (
        <div className="App">
            <header className="App-header">
                <p>{`${aFunction("testing")}`}</p>
                <a
                    className="App-link"
                    href="https://reactjs.org"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    Learn React
                </a>
            </header>
        </div>
    );
}

export default App;
