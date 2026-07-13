"use client";

import { useEffect } from "react";

export default function ScrollRestoration() {
  useEffect(() => {
    // Disable browser scroll restoration
    if (typeof window !== "undefined" && "scrollRestoration" in window.history) {
      window.history.scrollRestoration = "manual";
    }

    // Ensure page starts at top on initial load/refresh
    window.scrollTo(0, 0);
  }, []);

  return null;
}

