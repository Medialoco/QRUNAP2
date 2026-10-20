const $ = (id) => document.getElementById(id);
let lang =
  new URLSearchParams(location.search).get("lang") === "fr" ? "fr" : "en";
const text = {
  en: {
    guide: "Learning guide",
    eyebrow: "THE QUEUEING LAB · QNAP2 REVISITED",
    title: "Old language.<br>Fresh experiments.",
    lead: "Remember by doing. Follow a customer through a queue, change the model, and see what happens.",
    local: "Runs in your browser",
    localNote:
      "A Rust engine. No installation. Your model stays on your device.",
    subset: "Explore the supported subset →",
    choose: "01 / Pick an experiment",
    editable: "Every example is editable",
    model: "02 / Read & change the model",
    sourceLabel: "QNAP2 model source",
    editorHelp:
      "EXP(2) = mean duration 2 · CST(2) = fixed duration 2 · & starts a comment",
    seed: "Seed",
    run: "Run model ↗",
    stop: "Stop",
    reset: "Reset example",
    downloadSource: "Download .qnp",
    import: "Open .qnp",
    results: "03 / Understand the result",
    export: "Export JSON",
    population: "Customers in the network",
    chartNote:
      "201 samples across simulated time, including customers in service.",
    stationStats: "Station measurements",
    station: "Station",
    busy: "Utilization",
    throughput: "Throughput",
    definitions:
      "L: in station · Lq: waiting · W: station time · Wq: waiting time. W and Wq include completed visits only. Source utilization is not applicable.",
    readout: "HOW TO READ THIS",
    caveat:
      "One run, including startup. No confidence intervals or warm-up removal. Unfinished visits are excluded from W and Wq.",
    emptyTitle: "A little model. A useful question.",
    emptyText:
      "Run an example to see waiting times, utilization and the number of customers in the network.",
    refresher: "A QUICK REFRESHER",
    refresherTitle: "The names come back.<br>Then the intuition.",
    readGuide: "Read the step-by-step guide →",
    customer: "Customer",
    customerText:
      "The thing moving through the system: a request, a job or a person.",
    stationText:
      "A waiting queue and one or more servers that process its customers.",
    service: "Service",
    serviceText:
      "Processing time at a server; the interval between arrivals at a source.",
    footer:
      "An independent reconstruction. A small, documented subset of QNAP2.",
    sources: "Sources & historical changes",
    compatibility: "Compatibility (EN)",
    runExample: "Run this example ↗",
    original: "Original teaching model",
    historical: "Historical adaptation",
    ready: "Ready. Choose an experiment or edit the model below.",
    running: "Simulating locally…",
    stopped: "Stopped. You can edit the model and run again.",
    done: "Run complete",
    stale: "Model or seed changed — run again to update the results.",
    loading: "Loading example…",
    failed: "Unable to run this model. Engine diagnostic (English):",
    seedError: "Seed must be an integer between 1 and 4294967295.",
    remaining: "still in the network",
    exited: "customers exited",
    generated: "customers generated",
    duration: "simulated time units",
    events: "events",
    sourceNotice:
      "Original QRUNAP2 example. Runtime conventions and limits are documented.",
    historicNotice:
      "Reconstructed from INRIA QNAPREP (1995). Interactive input and parallel replications replaced. See the change record.",
    genericInsight:
      "Compare utilization with waiting. W includes service; Wq measures waiting only. Station times use the same unit as your SERVICE values. Check the guide before interpreting a finite run as equilibrium.",
    constantInsight:
      "For the unchanged example: arrivals are 2 units apart and service takes 1. There is no waiting. At time 20, the tenth arrival is still in service: 10 arrivals, 9 exits.",
    closedInsight:
      "For the unchanged historical adaptation: 40 customers circulate between A and B. No one enters or exits. Try slowing down B and watch where customers accumulate.",
    mm1Insight:
      "For the unchanged example, the long-run M/M/1 references are 50% utilization, L = 1 and W = 2. A finite simulation will differ. Change the seed to explore that variation.",
    multiInsight:
      "For the unchanged example, two servers share one queue. Each customer still requires a mean service time of 1; capacity comes from working in parallel.",
    overloadInsight:
      "For the unchanged example, arrivals average 1.25 per unit time but one server can handle only 1. The queue grows. There is no finite stationary M/M/1 mean.",
    imported: "Imported model",
    edited: "Edited model",
    sourceTooLarge: "Source exceeds 64 KiB.",
    chartLabel: "Network population over simulated time",
    time: "time",
    completed: "completed visits",
    loadError: "Could not load the example",
    cancelled: "Execution stopped",
    timeout: "Time limit reached. Reduce TMAX and try again.",
  },
  fr: {
    guide: "Guide de prise en main",
    eyebrow: "L’ATELIER DES FILES · QNAP2 REVISITÉ",
    title: "Retrouver le langage.<br>Reprendre l’expérience.",
    lead: "Retrouvez vos repères en pratiquant. Suivez un client dans une file, modifiez le modèle et observez le résultat.",
    local: "Dans votre navigateur",
    localNote:
      "Un moteur Rust. Rien à installer. Votre modèle reste sur votre appareil.",
    subset: "Découvrir le sous-ensemble pris en charge →",
    choose: "01 / Choisir une expérience",
    editable: "Tous les exemples sont modifiables",
    model: "02 / Lire et modifier le modèle",
    sourceLabel: "Code source du modèle QNAP2",
    editorHelp:
      "EXP(2) = durée moyenne 2 · CST(2) = durée fixe 2 · & commence un commentaire",
    seed: "Graine",
    run: "Exécuter ↗",
    stop: "Arrêter",
    reset: "Réinitialiser",
    downloadSource: "Télécharger .qnp",
    import: "Ouvrir .qnp",
    results: "03 / Comprendre le résultat",
    export: "Exporter JSON",
    population: "Clients dans le réseau",
    chartNote: "201 observations du temps simulé, clients en service compris.",
    stationStats: "Mesures par station",
    station: "Station",
    busy: "Occupation",
    throughput: "Débit",
    definitions:
      "L : dans la station · Lq : en attente · W : séjour · Wq : attente. W et Wq portent sur les visites terminées. L’occupation ne s’applique pas aux sources.",
    readout: "COMMENT LIRE CE RÉSULTAT",
    caveat:
      "Une seule exécution, démarrage inclus. Sans intervalle de confiance ni suppression du démarrage. Les visites inachevées sont exclues de W et Wq.",
    emptyTitle: "Un petit modèle. Une question utile.",
    emptyText:
      "Exécutez un exemple pour observer les attentes, l’occupation et le nombre de clients dans le réseau.",
    refresher: "QUELQUES REPÈRES",
    refresherTitle: "Les mots reviennent.<br>Puis l’intuition.",
    readGuide: "Lire le guide pas à pas →",
    customer: "Client",
    customerText:
      "Ce qui traverse le système : une requête, une tâche ou une personne.",
    stationText:
      "Une file d’attente et un ou plusieurs serveurs pour traiter les clients.",
    service: "Service",
    serviceText:
      "Le temps de traitement sur un serveur ; l’intervalle entre arrivées sur une source.",
    footer:
      "Une reconstruction indépendante. Un petit sous-ensemble documenté de QNAP2.",
    sources: "Sources et adaptations (EN)",
    compatibility: "Compatibilité (EN)",
    runExample: "Exécuter cet exemple ↗",
    original: "Exemple pédagogique original",
    historical: "Adaptation historique",
    ready: "Prêt. Choisissez une expérience ou modifiez le modèle.",
    running: "Simulation locale en cours…",
    stopped: "Arrêté. Vous pouvez modifier le modèle et relancer.",
    done: "Exécution terminée",
    stale:
      "Modèle ou graine modifiés : relancez pour actualiser les résultats.",
    loading: "Chargement de l’exemple…",
    failed:
      "Impossible d’exécuter ce modèle. Diagnostic du moteur (en anglais) :",
    seedError: "La graine doit être un entier entre 1 et 4294967295.",
    remaining: "encore dans le réseau",
    exited: "clients sortis",
    generated: "clients créés",
    duration: "unités de temps simulé",
    events: "événements",
    sourceNotice:
      "Exemple original QRUNAP2. Les conventions et limites sont documentées.",
    historicNotice:
      "Reconstruction de QNAPREP, INRIA (1995). Saisie interactive et réplications parallèles remplacées. Voir les modifications.",
    genericInsight:
      "Comparez l’occupation et l’attente. W inclut le service ; Wq mesure seulement l’attente. Les temps utilisent l’unité de SERVICE. Consultez le guide avant d’interpréter une simulation finie comme un équilibre.",
    constantInsight:
      "Pour l’exemple inchangé : les arrivées sont espacées de 2 unités et le service dure 1. Il n’y a pas d’attente. À t=20, le dixième client est encore en service : 10 arrivées, 9 sorties.",
    closedInsight:
      "Pour l’adaptation historique inchangée : 40 clients circulent entre A et B, sans entrée ni sortie. Ralentissez B et observez où les clients s’accumulent.",
    mm1Insight:
      "Pour l’exemple inchangé, les références M/M/1 à long terme sont 50 % d’occupation, L = 1 et W = 2. Une simulation finie s’en écarte. Changez la graine pour explorer cette variation.",
    multiInsight:
      "Pour l’exemple inchangé, deux serveurs partagent une file. Chaque service dure toujours 1 unité en moyenne ; la capacité provient du travail en parallèle.",
    overloadInsight:
      "Pour l’exemple inchangé, 1,25 client arrive par unité de temps, mais le serveur n’en traite que 1 en moyenne. La file grandit. Il n’existe pas de moyenne stationnaire M/M/1 finie.",
    imported: "Modèle importé",
    edited: "Modèle modifié",
    sourceTooLarge: "Le source dépasse 64 Kio.",
    chartLabel: "Population du réseau au fil du temps simulé",
    time: "temps",
    completed: "visites terminées",
    loadError: "Impossible de charger l’exemple",
    cancelled: "Exécution arrêtée",
    timeout: "Limite de temps atteinte. Réduisez TMAX et réessayez.",
  },
};
const examples = [
  {
    id: "first-queue",
    path: "original/first-queue.qnp",
    en: "The first queue",
    fr: "La première file",
    description: {
      en: "One desk, irregular arrivals. Why can waiting happen at 50% utilization?",
      fr: "Un guichet, des arrivées irrégulières. Pourquoi attendre avec 50 % d’occupation ?",
    },
    insight: "mm1Insight",
  },
  {
    id: "deterministic",
    path: "original/deterministic.qnp",
    en: "Constant durations",
    fr: "Durées constantes",
    description: {
      en: "Make every interval predictable. Follow a short run you can check by hand.",
      fr: "Rendez chaque durée prévisible. Vérifiez une courte simulation à la main.",
    },
    insight: "constantInsight",
  },
  {
    id: "two-servers",
    path: "original/two-servers.qnp",
    en: "A second server",
    fr: "Un second serveur",
    description: {
      en: "Two servers, one shared queue. Explore capacity without making service faster.",
      fr: "Deux serveurs, une file. Augmentez la capacité sans accélérer le service.",
    },
    insight: "multiInsight",
  },
  {
    id: "overload",
    path: "original/overload.qnp",
    en: "Too much traffic",
    fr: "Trop d’arrivées",
    description: {
      en: "Requests arrive faster than they can be served. Watch the backlog grow.",
      fr: "Les clients arrivent plus vite qu’ils ne sont servis. Observez l’accumulation.",
    },
    insight: "overloadInsight",
  },
  {
    id: "inria-1995-adapted",
    path: "historical/inria-1995-adapted.qnp",
    en: "INRIA, 1995",
    fr: "INRIA, 1995",
    historical: true,
    description: {
      en: "A real historical topology, adapted: 40 customers circulating between two stations.",
      fr: "Une topologie historique adaptée : 40 clients circulant entre deux stations.",
    },
    insight: "closedInsight",
  },
];
let current = examples[0],
  original = "",
  report = null,
  worker = null,
  timer = null,
  loadVersion = 0,
  statusKey = "ready",
  lastSource = "";
const t = (key) => text[lang][key] || key;
function status(key) {
  statusKey = key;
  $("status").textContent = t(key);
}
function number(value, digits = 3) {
  return value == null
    ? "—"
    : new Intl.NumberFormat(lang, { maximumFractionDigits: digits }).format(
        value,
      );
}
function stopWorker() {
  worker?.terminate();
  worker = null;
  clearTimeout(timer);
  $("run").disabled = false;
  $("stop").disabled = true;
}
function clearResult() {
  report = null;
  $("export").disabled = true;
  $("results").hidden = true;
  $("empty").hidden = false;
  $("error").hidden = true;
}
function markDirty() {
  clearResult();
  status("stale");
}
function renderExampleInfo() {
  $("example-description").textContent = current
    ? current.description[lang]
    : t("imported");
  $("provenance").replaceChildren();
  if (current) {
    $("provenance").append(
      document.createTextNode(
        t(current.historical ? "historicNotice" : "sourceNotice") + " ",
      ),
    );
    const a = document.createElement("a");
    a.href = "docs/sources.html";
    a.textContent = t("sources");
    $("provenance").append(a);
  }
}
function translate() {
  document.documentElement.lang = lang;
  document.querySelectorAll("[data-i18n]").forEach((el) => {
    const value = t(el.dataset.i18n);
    if (["title", "refresherTitle"].includes(el.dataset.i18n))
      el.innerHTML = value;
    else el.textContent = value;
  });
  $("language").textContent = lang === "en" ? "FR" : "EN";
  $("language").setAttribute(
    "aria-label",
    lang === "en" ? "Passer en français" : "Switch to English",
  );
  $("guide-link").href = $("bottom-guide").href =
    `docs/guide${lang === "fr" ? ".fr" : ""}.html`;
  $("examples").replaceChildren();
  examples.forEach((ex) => {
    const b = document.createElement("button");
    b.className = "example" + (current === ex ? " active" : "");
    b.setAttribute("aria-pressed", String(current === ex));
    for (const [cls, value] of [
      ["example-label", t(ex.historical ? "historical" : "original")],
      ["example-title", ex[lang]],
      ["example-cta", t("runExample")],
    ]) {
      const s = document.createElement("span");
      s.className = cls;
      s.textContent = value;
      b.append(s);
    }
    b.addEventListener("click", () => loadExample(ex, true));
    $("examples").append(b);
  });
  renderExampleInfo();
  status(statusKey);
  if (report) renderReport();
}
async function loadExample(ex, autorun = false) {
  const version = ++loadVersion;
  stopWorker();
  clearResult();
  status("loading");
  $("run").disabled = true;
  try {
    const response = await fetch(`examples/${ex.path}`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const source = await response.text();
    if (version !== loadVersion) return;
    current = ex;
    original = source;
    $("source").value = source;
    status("ready");
    translate();
    if (autorun) run();
    else $("run").disabled = false;
  } catch (e) {
    if (version !== loadVersion) return;
    $("run").disabled = false;
    showError(`${t("loadError")}: ${e.message}`);
  }
}
function showError(message) {
  stopWorker();
  clearResult();
  status("failed");
  $("error").hidden = false;
  $("error").textContent = message;
}
function run() {
  stopWorker();
  clearResult();
  const seed = Number($("seed").value);
  if (!Number.isInteger(seed) || seed < 1 || seed > 4294967295) {
    showError(t("seedError"));
    return;
  }
  if (new TextEncoder().encode($("source").value).length > 65536) {
    showError(t("sourceTooLarge"));
    return;
  }
  lastSource = $("source").value;
  status("running");
  $("run").disabled = true;
  $("stop").disabled = false;
  worker = new Worker("worker.js");
  worker.onmessage = ({ data }) => {
    stopWorker();
    if (!data.ok) {
      showError(data.error);
      return;
    }
    report = data.report;
    status("done");
    renderReport();
  };
  worker.onerror = (e) => showError(e.message);
  worker.postMessage({ source: lastSource, seed });
  timer = setTimeout(() => showError(t("timeout")), 20000);
}
function renderReport() {
  $("results").hidden = false;
  $("empty").hidden = true;
  $("export").disabled = false;
  $("status").textContent =
    `${t("done")} · ${number(report.events, 0)} ${t("events")} · ${t("seed")} ${report.seed}`;
  $("metrics").replaceChildren();
  for (const [value, label] of [
    [report.horizon, "duration"],
    [report.exited, "exited"],
    [report.remaining, "remaining"],
  ]) {
    const box = document.createElement("div");
    box.className = "metric";
    const strong = document.createElement("strong");
    strong.textContent = number(value);
    const span = document.createElement("span");
    span.textContent = t(label);
    box.append(strong, span);
    $("metrics").append(box);
  }
  const rows = $("station-rows");
  rows.replaceChildren();
  for (const s of report.stations) {
    const row = document.createElement("tr");
    for (const value of [
      s.name,
      s.source ? "—" : `${number(s.utilization * 100, 1)}%`,
      s.source ? "—" : number(s.mean_number),
      s.source ? "—" : number(s.mean_queue),
      number(s.mean_response),
      number(s.mean_wait),
      number(s.throughput),
    ]) {
      const cell = document.createElement("td");
      cell.textContent = value;
      row.append(cell);
    }
    rows.append(row);
  }
  const max = Math.max(1, ...report.trace.map((p) => p.population));
  const width = 480,
    height = 175,
    left = 35,
    right = 13,
    top = 15,
    bottom = 30;
  const x = (v) => left + ((width - left - right) * v) / report.horizon,
    y = (v) => height - bottom - ((height - top - bottom) * v) / max;
  const points = report.trace
    .map((p) => `${x(p.time)},${y(p.population)}`)
    .join(" ");
  const ns = "http://www.w3.org/2000/svg";
  const svg = document.createElementNS(ns, "svg");
  svg.setAttribute("viewBox", `0 0 ${width} ${height}`);
  svg.setAttribute("role", "img");
  svg.setAttribute("aria-label", t("chartLabel"));
  function node(tag, attrs, label) {
    const el = document.createElementNS(ns, tag);
    Object.entries(attrs).forEach(([k, v]) => el.setAttribute(k, v));
    if (label !== undefined) el.textContent = label;
    svg.append(el);
  }
  for (const v of [0, max / 2, max]) {
    node("line", {
      x1: left,
      y1: y(v),
      x2: width - right,
      y2: y(v),
      stroke: "#d6e0d9",
      "stroke-width": 1,
    });
    node(
      "text",
      {
        x: left - 6,
        y: y(v) + 4,
        "text-anchor": "end",
        fill: "#53666e",
        "font-size": 10,
      },
      number(v, 1),
    );
  }
  node("polygon", {
    points: `${left},${y(0)} ${points} ${width - right},${y(0)}`,
    fill: "#c7ddd2",
    opacity: 0.7,
  });
  node("polyline", {
    points,
    fill: "none",
    stroke: "#126a65",
    "stroke-width": 2,
  });
  for (const v of [0, report.horizon / 2, report.horizon])
    node(
      "text",
      {
        x: x(v),
        y: height - 10,
        "text-anchor":
          v === 0 ? "start" : v === report.horizon ? "end" : "middle",
        fill: "#53666e",
        "font-size": 10,
      },
      number(v),
    );
  $("chart").replaceChildren(svg);
  $("insight").textContent = t(
    current && lastSource === original ? current.insight : "genericInsight",
  );
}
function download(name, content, type) {
  const a = document.createElement("a");
  a.href = URL.createObjectURL(new Blob([content], { type }));
  a.download = name;
  a.click();
  setTimeout(() => URL.revokeObjectURL(a.href), 1000);
}
$("language").onclick = () => {
  lang = lang === "en" ? "fr" : "en";
  const url = new URL(location);
  if (lang === "fr") url.searchParams.set("lang", "fr");
  else url.searchParams.delete("lang");
  history.replaceState(null, "", url);
  translate();
};
$("run").onclick = run;
$("stop").onclick = () => {
  stopWorker();
  clearResult();
  status("stopped");
};
$("reset").onclick = () => {
  loadExample(current || examples[0]);
};
$("source").addEventListener("input", () => {
  ++loadVersion;
  stopWorker();
  markDirty();
});
$("seed").addEventListener("input", () => {
  stopWorker();
  markDirty();
});
$("source").addEventListener("keydown", (e) => {
  if ((e.ctrlKey || e.metaKey) && e.key === "Enter") {
    e.preventDefault();
    run();
  }
});
$("download-source").onclick = () =>
  download("model.qnp", $("source").value, "text/plain");
$("export").onclick = () => {
  if (report)
    download(
      "qrunap2-result.json",
      JSON.stringify({ source: lastSource, ...report }, null, 2),
      "application/json",
    );
};
$("import").onchange = async (e) => {
  const f = e.target.files[0];
  if (!f) return;
  const version = ++loadVersion;
  stopWorker();
  if (f.size > 65536) {
    showError(t("sourceTooLarge"));
    return;
  }
  const source = await f.text();
  if (version !== loadVersion) return;
  current = null;
  original = "";
  $("source").value = source;
  markDirty();
  translate();
  e.target.value = "";
};
translate();
loadExample(examples[0]);
