const { invoke } = window.__TAURI__.core;

window.addEventListener("DOMContentLoaded", () => {
  const display = document.getElementById("display");
  const buttons = Array.from(document.getElementsByClassName("btn"));
  let currentExpression = "";

  buttons.map((button) => {
    button.addEventListener("click", (e) => {
      const value = e.target.innerText;

      if (value === "C") {
        currentExpression = "";
        display.innerText = "0";
        return;
      }

      if (value === "=") {
        invoke("calculate", { expression: currentExpression })
          .then((result) => {
            display.innerText = result;
            currentExpression = result.toString();
          })
          .catch((error) => {
            display.innerText = "Error";
            currentExpression = "";
          });
        return;
      }

      if (display.innerText === "0" && value !== ".") {
        currentExpression = value;
      } else {
        currentExpression += value;
      }
      display.innerText = currentExpression;
    });
  });
});
