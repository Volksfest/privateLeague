var player = document.querySelector("#player");
            var match = document.querySelector("#match");
var popup = document.querySelector("#popup");

var reg = /^([1-9]?[0-9]):([0-5][0-9])$/;

console.log("Wololo");

var matches = document.querySelectorAll(".played, .ongoing, .unplayed");
matches.forEach(match => {
match.addEventListener("click", openPopup, true);
match.addEventListener("contextmenu", openContext);
});


// Thx Stackoverflow
function getWeekNumber(){
    var d = new Date();
    var dayNum = d.getUTCDay() || 7;
    d.setUTCDate(d.getUTCDate() + 4 - dayNum);
    var yearStart = new Date(Date.UTC(d.getUTCFullYear(),0,1));
    return Math.ceil((((d - yearStart) / 86400000) + 1)/7);
};

function dateToString(date) {
    return date.getDate() + "." + (date.getMonth() + 1) + ".";
}

function getWeekAnalysis(week) {
    var yearStart = new Date(Date.UTC((new Date()).getUTCFullYear(),0,1));
    var days_to_monday = (yearStart.getUTCDay() + 6) %7;
    yearStart.setDate(yearStart.getDate() - days_to_monday);
    var startDate = new Date(yearStart);
    var endDate = new Date(yearStart);
    startDate.setDate(startDate.getDate() + (7 * (week - 1)) );
    endDate.setDate(endDate.getDate() + (7 * week - 1));
    return {
        "start_date": new Date(startDate),
        "end_date": new Date(endDate),
        };
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
    document.getElementById("second_player_label").innerHTML= inner[1];
}

function openContext(e) {
    console.log(this);
    var inner = this.innerText.split("\n");
    if (confirm("Delete the games of the match '" +inner[0]+" vs. "+inner[1]+"'?")) {
        json = JSON.stringify({"RemoveGames":{
        "player1": inner[0],
        "player2": inner[1],
        }});
        websocket.send(json);
    }
    e.preventDefault();
    return false;
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
    websocket.send(json);
    hidePopup();
}
