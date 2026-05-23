const API_BASE = "http://localhost:3001";

async function apiRequest(endpoint, options = {}) {
  const url = `${API_BASE}${endpoint}`;
  const config = { headers: { "Content-Type": "application/json" }, ...options };
  const res = await fetch(url, config);
  return { status: res.status, data: await res.json().catch(() => null) };
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
