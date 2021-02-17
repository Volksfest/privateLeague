var player = document.querySelector("#player");
            var match = document.querySelector("#match");
var popup = document.querySelector("#popup");

var reg = /^([1-9]?[0-9]):([0-5][0-9])$/;

var matches = document.querySelectorAll(".played, .ongoing, .unplayed");

matches.forEach(match => {
    match.addEventListener("click", openPopup, true);
    match.addEventListener("contextmenu", openContext);
});

function openContext(e) {
    var inner = this.innerText.split("\n");
    if (confirm("Delete the games of the match '" +inner[0]+" vs. "+inner[2]+"'?")) {
        json = JSON.stringify({"RemoveGames":{
        "player1": inner[0],
        "player2": inner[2],
        }});

        sendJson(json);
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

    json = JSON.stringify({"AddGame":{
        "first_player_win": document.querySelector('input[name="first_player_won"]:checked').value == "true",
        "player1": [document.getElementById("first_player_label").innerText,
                    race1.charAt(0).toLowerCase()],
        "player2": [document.getElementById("second_player_label").innerText,
                    race2.charAt(0).toLowerCase()],
        "duration_min" : parseInt(match[1]),
        "duration_sec" : parseInt(match[2])
    }});

    hidePopup();
    sendJson(json);
}

function sendJson(json) {
    fetch("/api", {
            method:'POST',
            headers: {'Content-Type': 'application/json'},
            body: json
    })
    .then(response => response.json())
    .then(json => parseJson(json));
}

function parseJson(json) {
    if (json.hasOwnProperty("Update")) {
        var elementBuilder = document.createElement("template");
        elementBuilder.innerHTML = json.Update.dom.trim();
        var newElement = elementBuilder.content.firstChild;
        var oldElement = document.getElementById("match_" + json.Update.idx);

        newElement.addEventListener("click", openPopup, true);
        newElement.addEventListener("contextmenu", openContext);

        oldElement.parentNode.replaceChild(newElement, oldElement);
    }
}
