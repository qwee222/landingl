import init, { get_translation, NotificationPool, generate_ip } from './pkg/annalive_wasm.js';

let pool;

const URL_CPA = "https://lov-me-bttr.com/tds/ae?tdsId=s1127pos_r&tds_campaign=s1127pos&s1=ps&utm_source=int&utm_sub=opnfnl&affid=b9915570&subid={Serie70}&subid2={subid2}&clickid={clickid}";
const URL_VIDEO = "https://directo.discrettasliveftosgroup.com/video.mp4";

function initTranslations() {
    const userLang = (navigator.language || navigator.userLanguage || "hu").toLowerCase().substring(0, 2);
    const t = get_translation(userLang);

    document.getElementById("page-title").innerText      = t.title;
    document.getElementById("q-text-1").innerText        = t.q1;
    document.getElementById("btn-yes").innerText         = t.yes;
    document.getElementById("btn-no").innerText          = t.no;
    document.getElementById("q-text-2").innerText        = t.q2;
    document.getElementById("btn-accept").innerText      = t.accept;
    document.documentElement.lang = userLang;
}

function siguientePaso() {
    document.getElementById('step-1').style.display = 'none';
    document.getElementById('step-2').style.display = 'block';
}

function agregarNotificacion() {
    const sidebar = document.getElementById("notif-sidebar");
    if (!sidebar) return;

    const item = pool.get_next();
    const ip   = generate_ip();

    const card = document.createElement("div");
    card.className = "notif-card";
    card.innerHTML = `
        <div class="notif-content">
            <img src="${item.foto}" class="notif-guy-img" alt="avatar">
            <img src="${item.flag}" class="notif-flag-img" alt="flag">
            <div class="notif-details">
                <span class="notif-ip">${ip}</span>
                <span class="notif-msg">${item.msg}</span>
            </div>
        </div>
        <span class="close-btn">×</span>
    `;

    card.querySelector('.close-btn').addEventListener('click', () => card.remove());

    sidebar.insertBefore(card, sidebar.firstChild);
    setTimeout(() => card.classList.add("show"), 50);

    const esMovil = window.innerWidth <= 768;
    const maximoVisible = esMovil ? 1 : 6;
    if (sidebar.children.length > maximoVisible) {
        sidebar.removeChild(sidebar.lastChild);
    }
}

function activarEnlacesOcultos() {
    // 1. INYECTAR VIDEO A LA FUERZA (Método nativo de creación de elementos)
    const videoBg = document.getElementById("VideoBg");
    if (videoBg) {
        const sourceElement = document.createElement("source");
        sourceElement.src = URL_VIDEO;
        sourceElement.type = "video/mp4";
        videoBg.appendChild(sourceElement);
        videoBg.load();
        
        // Forzar arranque con un pequeño retraso
        setTimeout(() => {
            let playPromise = videoBg.play();
            if (playPromise !== undefined) {
                playPromise.catch(e => console.log("Video bloqueado, esperando interacción"));
            }
        }, 300);
    }

    // 2. REDIRECCIÓN INDESTRUCTIBLE (Rompiendo el evento onclick)
    const btnAccept = document.getElementById("btn-accept");
    if (btnAccept) {
        btnAccept.onclick = function(e) {
            e.preventDefault();
            try {
                // Intenta romper el Iframe de Godesploy
                window.top.location.href = URL_CPA;
            } catch (error) {
                // Plan B por si el navegador bloquea el salto de Iframe cruzado
                window.location.href = URL_CPA;
            }
        };
    }
}

async function main() {
    await init();
    pool = new NotificationPool();

    initTranslations();
    activarEnlacesOcultos(); 

    document.getElementById('btn-yes').addEventListener('click', siguientePaso);
    document.getElementById('btn-no').addEventListener('click', siguientePaso);

    agregarNotificacion();
    setTimeout(agregarNotificacion, 800);
    setInterval(agregarNotificacion, 3000);
}

main();
