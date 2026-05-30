const API_BASE = "http://localhost:3001";

async function apiRequest(endpoint, options = {}) {
  const url = `${API_BASE}${endpoint}`;
  const config = { headers: { "Content-Type": "application/json" }, ...options };
  try {
    const res = await fetch(url, config);
    const text = await res.text();
    let data = null;
    try {
      data = JSON.parse(text);
    } catch {
      data = null;
    }
    return { status: res.status, data };
  } catch (error) {
    console.error("API request failed", error);
    return { status: 0, data: null, error: error?.message || "Erro de rede" };
  }
}

function getUser() {
  const u = localStorage.getItem("user");
  return u ? JSON.parse(u) : null;
}

function requireAdmin() {
  const u = getUser();
  if (!u || u.user_type !== "Admin") {
    window.location.href = "../login.html";
    return null;
  }
  return u;
}

function requireGerente() {
  const u = getUser();
  if (!u || u.user_type !== "Gerente") {
    window.location.href = "../login.html";
    return null;
  }
  return u;
}

function requireAuth(allowedTypes) {
  const u = getUser();
  if (!u || !allowedTypes.includes(u.user_type)) {
    window.location.href = "../login.html";
    return null;
  }
  return u;
}

function logout() {
  localStorage.removeItem("user");
  window.location.href = "../login.html";
}

function roleBadge(type) {
  const cls = type === "Admin" ? "badge-admin" : type === "Gerente" ? "badge-gerente" : "badge-funcionario";
  return `<span class="badge ${cls}">${type}</span>`;
}
