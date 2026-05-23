function renderSidebarGerente(active) {
  const links = [
    { href: "dashboard.html", icon: "📊", label: "Dashboard", id: "dashboard" },
    { section: "Produtos" },
    { href: "products.html", icon: "📋", label: "Listar Produtos", id: "products" },
    { href: "product-create.html", icon: "➕", label: "Cadastrar Produto", id: "product-create" },
    { section: "Estoque" },
    { href: "stock-entry.html", icon: "📥", label: "Entrada de Estoque", id: "stock-entry" },
    { href: "stock-exit.html", icon: "📤", label: "Saída de Estoque", id: "stock-exit" },
    { href: "stock-search.html", icon: "🔍", label: "Consultar Estoque", id: "stock-search" },
    { section: "Movimentações" },
    { href: "movements.html", icon: "🔄", label: "Histórico", id: "movements" },
    { section: "Relatórios" },
    { href: "report-stock.html", icon: "📦", label: "Relatório de Estoque", id: "report-stock" },
    { href: "report-critical.html", icon: "🚨", label: "Estoque Crítico", id: "report-critical" },
    { href: "report-alerts.html", icon: "⚠️", label: "Alertas de Consumo", id: "report-alerts" },
    { href: "report-low-stock.html", icon: "📉", label: "Estoque Baixo", id: "report-low-stock" },
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
