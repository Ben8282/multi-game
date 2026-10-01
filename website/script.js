const docker_button = document.getElementById("docker-button");

if (docker_button){
docker_button.onclick = function(){
    window.location.href = "docker.html";
};
}

const releases_button = document.getElementById("releases-button");

if (releases_button){
releases_button.onclick = function(){
    window.location.href = "https://github.com/Ben8282/multi-game/releases";
};
}

const sourceButton = document.getElementById("source-button");

if (sourceButton){
sourceButton.onclick = function(){
    window.location.href = "https://github.com/Ben8282/multi-game";
};
}

const go_back_button = document.getElementById("go-back-button");

if (go_back_button){
go_back_button.onclick = function(){
    window.location.href = "index.html";
};
}

document.getElementById("year").textContent = new Date().getFullYear();