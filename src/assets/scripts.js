// scripts.js
// -- BLOCK: GLOBAL_VARIABLE
const HISTORY_DRAWER_W = "20rem";
let historyDrawer = undefined;
let historyDrawerIsShow = true;
let newsSummarizeEvtSource = undefined;
if (newsSummarizeEvtSource) newsSummarizeEvtSource.close();
let summaryReqUrl = "";
let analyzeSearchInput = undefined;
// -- ENDBLOCK: GLOBAL_VARIABLE

function toggleDrawer(drawerButton) {
  if (historyDrawerIsShow) {
    // HIDE IT
    historyDrawer.style.transform = `translateX(-${HISTORY_DRAWER_W})`;
    drawerButton.nextElementSibling.classList.replace(
      "text-neutral-200",
      "text-neutral-800",
    );

    drawerButton.parentNode.classList.replace("bg-gray-800", "bg-transparent");
  } else {
    historyDrawer.style.transform = `translateX(${HISTORY_DRAWER_W})`;
    drawerButton.nextElementSibling.classList.replace(
      "text-neutral-800",
      "text-neutral-200",
    );
    drawerButton.parentNode.classList.replace("bg-transparent", "bg-gray-800");
  }

  historyDrawerIsShow = !historyDrawerIsShow;
}

function loadNewsHistory(historyItemAnchor) {
  let analyzeInput = document.querySelector("#analyze-search-input");

  analyzeInput.value = historyItemAnchor.getAttribute("data-history-url");
  toggleDrawer(document.querySelector("#drawer-button"));
}
