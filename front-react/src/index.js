import React from "react";
import ReactDOM from "react-dom";

function bigComputation() {
  alert("Big Computation in JS");
}

const App = () => {
  return (
    <div>
      <h1>MOVIES RR2</h1>
      <button onClick={bigComputation}>Run Computation</button>
    </div>
  );
};

ReactDOM.render(
  <App/>,
  document.getElementById("root")
);
