const docker_button = document.getElementById("docker-button");

docker_button.onclick = function(){
    window.location.href = "docker.html";
};

const releases_button = document.getElementById("releases-button");

releases_button.onclick = function(){
    window.location.href = "https://github.com/Ben8282/multi-game/releases";
};

const sourceButton = document.getElementById("Source-button");

sourceButton.onclick = function(){
    window.location.href = "https://github.com/Ben8282/multi-game";
};

const readme = document.getElementById("readme");

readme.innerHTML = marked.parse(readme.textContent);