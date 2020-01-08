
const rust = import("./pkg/index");

rust.then((modul) => {

    const execFact = () => {
        const num = Number(document.getElementById("number-field").value);
        const result = modul.fact(num);

        const h1 = document.createElement("h1");
        h1.appendChild(document.createTextNode(`${num}! = ${result}`));
        document.body.appendChild(h1);
    }

    document.getElementById("button").onclick = execFact;

    modul.log_with_rust("Ready to rumble!");
})