import init, { get_translation, NotificationPool, generate_ip } from './pkg/annalive_wasm.js';

let pool;

function initTranslations() {
    const userLang = (navigator.language || navigator.userLanguage || "hu").toLowerCase().substring(0, 2);
    const t = get_translation(userLang);

    document.getElementById("page-title").innerText      = t.title;
    document.getElementById("q-text-1").innerText          = t.q1;
    document.getElementById("btn-yes").innerText           = t.yes;
    document.getElementById("btn-no").innerText            = t.no;
    document.getElementById("q-text-2").innerText          = t.q2;
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

async function main() {
    await init();              // carga el .wasm
    pool = new NotificationPool();

    initTranslations();

    document.getElementById('btn-yes').addEventListener('click', siguientePaso);
    document.getElementById('btn-no').addEventListener('click', siguientePaso);

    agregarNotificacion();
    setTimeout(agregarNotificacion, 800);
    setInterval(agregarNotificacion, 3000);
}

main();
