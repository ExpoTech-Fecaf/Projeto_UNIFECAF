function renderSidebarFuncionario(active) {
  const links = [
    { href: "dashboard.html", icon: "📊", label: "Dashboard", id: "dashboard" },
    { section: "Estoque" },
    { href: "stock-exit.html", icon: "📤", label: "Retirada de Estoque", id: "stock-exit" },
    { href: "stock-search.html", icon: "🔍", label: "Consultar Estoque", id: "stock-search" },
  ];

  let html = `<img src="../assets/logo.jpeg" alt="FoodStock" class="sidebar-logo"><nav>`;
  for (const l of links) {
    if (l.section) {
      html += `<div class="nav-section">${l.section}</div>`;
    } else {
      html += `<a href="${l.href}" class="${l.id === active ? 'active' : ''}">${l.icon} ${l.label}</a>`;
    }
  }
  html += `</nav><div class="logout"><a href="#" onclick="logout()" style="color:#f87171;">🚪 Sair</a></div>`;
  document.getElementById("sidebar").innerHTML = html;
}
