var player = document.querySelector("#player");
            var match = document.querySelector("#match");
var popup = document.querySelector("#popup");

var reg = /^([1-9]?[0-9]):([0-5][0-9])$/;

var matches = document.querySelectorAll(".played, .ongoing, .unplayed");

matches.forEach(match => {
    match.addEventListener("click", openPopup, true);
    match.addEventListener("contextmenu", openContext);
});

var token = 0;
update_token();

window.setInterval(() => console.log(token), 3000);

function openContext(e) {
    var inner = this.innerText.split("\n");
    if (confirm("Delete the games of the match '" +inner[0]+" vs. "+inner[2]+"'?")) {
        obj = {"RemoveGames":{
        "player1": inner[0],
        "player2": inner[2],
        }};

        sendJson(obj);
    }
    e.preventDefault();
    return false;
}

function openPopup(e) {
    if (popup.className == "active") {
        return;
    }

    this.appendChild(popup);
    document.getElementById("error_output").innerHTML = "";
    // TODO check size and make correct positioning
    popup.className="active";
    var inner = this.innerText.split("\n");
    document.getElementById("first_player_label").innerHTML= inner[0];
    document.getElementById("second_player_label").innerHTML= inner[2];
}

function hidePopup() {
    popup.className = "hidden";
}

function addGame() {
    // Control checks:
    var race1 = document.getElementById("first_player_race_input").value;
    var race2 = document.getElementById("second_player_race_input").value;
    var time = document.getElementById("duration_text").value;
    var match = reg.exec(time);

    var error = document.getElementById("error_output");

    if (document.querySelector("#race option[value='" + race1 + "']") == null) {
        error.innerHTML = "Rasse für Spieler 1...";
        return;
    }
    if (document.querySelector("#race option[value='" + race2 + "']") == null) {
        error.innerHTML = "Rasse für Spieler 2...";
        return;
    }
    if (match == null) {
        error.innerHTML = "Spieldauer...";
        return;
    }

    var min = match[1];
    var sec = match[2];

    if (min == 0 && sec == 0) {
        error.innerHTML = "Spieldauer...";
        return;
    }

    obj = {"AddGame":{
        "first_player_win": document.getElementById("first_player_win_radio").checked,
        "player1": [document.getElementById("first_player_label").innerText,
                    race1.charAt(0).toLowerCase()],
        "player2": [document.getElementById("second_player_label").innerText,
                    race2.charAt(0).toLowerCase()],
        "duration_min" : parseInt(min),
        "duration_sec" : parseInt(sec)
    }};

    hidePopup();
    sendJson(obj);
}

function sendJson(obj) {
    var sendObj = {cmd: obj, token: token};

    var json = JSON.stringify(sendObj);

    fetch("/api", {
            method:'POST',
            headers: {'Content-Type': 'application/json'},
            body: json
    })
    .then(response => response.json())
    .then(json => parseJson(json));
}

function parseJson(json) {
    update_token();

    if (json.hasOwnProperty("Update")) {
        json.Update.matches.forEach(element => {
            replace_div("match_" + element.idx, element.dom, true);
        });
        replace_div("player", json.Update.table_dom, false);
        console.log(json.Update.processed)
    }
}

function replace_div(id, div, addListener) {
        var elementBuilder = document.createElement("template");
        elementBuilder.innerHTML = div.trim();
        var newElement = elementBuilder.content.firstChild;
        var oldElement = document.getElementById(id);

        if (addListener) {
            newElement.addEventListener("click", openPopup, true);
            newElement.addEventListener("contextmenu", openContext);
        }

        oldElement.parentNode.replaceChild(newElement, oldElement);
}

function update_token() {
    fetch("/get_token")
    .then(response => response.json())
    .then(json => { token = json.Token } );
}