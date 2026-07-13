"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { useEffect, useState } from "react";
import { createPortal } from "react-dom";
import { HelixQLogoMark } from "./icons";

const navLinks = [
  { label: "Home", href: "/" },
  { label: "Search", href: "/#search" },
  { label: "API", href: "/#api" },
  { label: "CLI", href: "/#cli" },
  { label: "Docs", href: "/#docs" },
  { label: "About", href: "/#about" },
];

function NavToggle({
  open,
  onClick,
}: {
  open: boolean;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      className={`raycast-nav-toggle ${open ? "raycast-nav-toggle-open" : ""}`}
      onClick={onClick}
      aria-label={open ? "Close menu" : "Open menu"}
      aria-expanded={open}
    >
      <span className="raycast-nav-toggle-line" />
      <span className="raycast-nav-toggle-line" />
      <span className="raycast-nav-toggle-line" />
    </button>
  );
}

function handleNavClick(event: React.MouseEvent<HTMLAnchorElement>, href: string) {
  const hashIndex = href.indexOf("#");
  if (hashIndex === -1) {
    return;
  }

  const hash = href.slice(hashIndex + 1);
  if (!hash) return;

  const target = document.getElementById(hash);
  // No section rendered yet — keep the page where it is.
  if (!target) {
    event.preventDefault();
    return;
  }

  // Target exists but is already on-screen — focus without scrolling.
  event.preventDefault();
  const rect = target.getBoundingClientRect();
  const inView = rect.top >= 0 && rect.bottom <= window.innerHeight;
  if (!inView) {
    target.scrollIntoView({ behavior: "smooth", block: "nearest" });
  }

  if (hash === "search") {
    document.getElementById("helixq-query")?.focus({ preventScroll: true });
  }

  window.history.replaceState(null, "", `#${hash}`);
}

export default function RaycastNavbar() {
  const [menuOpen, setMenuOpen] = useState(false);
  const [mounted, setMounted] = useState(false);
  const pathname = usePathname();

  useEffect(() => {
    setMounted(true);
  }, []);

  const navbar = (
    <div
      id="navbar"
      className={`raycast-navbar-container ${menuOpen ? "raycast-navbar-container-open" : ""}`}
    >
      <div className={`raycast-navbar ${menuOpen ? "raycast-navbar-expanded" : ""}`}>
        <div className="raycast-navbar-top">
          <Link href="/" className="raycast-navbar-logo" aria-label="HelixQ home">
            <HelixQLogoMark className="raycast-navbar-logo-image helixq-logo-mark" />
            <span className="helixq-logo-wordmark">HelixQ</span>
          </Link>
          <NavToggle open={menuOpen} onClick={() => setMenuOpen((open) => !open)} />
        </div>

        <nav className="raycast-navbar-links" aria-label="Main navigation">
          {navLinks.map((link) => {
            const active =
              link.href === "/"
                ? pathname === "/"
                : pathname === link.href;
            return (
              <Link
                key={link.label}
                href={link.href}
                className={`raycast-nav-link ${active && link.href === "/" ? "raycast-nav-link-active" : ""}`}
                onClick={(event) => {
                  setMenuOpen(false);
                  handleNavClick(event, link.href);
                }}
              >
                {link.label}
              </Link>
            );
          })}
        </nav>

        <div className="raycast-navbar-actions">
          <div className="raycast-navbar-actions-wide">
            <Link
              href="/#docs"
              className="raycast-nav-link"
              onClick={(event) => {
                setMenuOpen(false);
                handleNavClick(event, "/#docs");
              }}
            >
              Docs
            </Link>
            <Link
              href="/#search"
              className="raycast-btn-light"
              onClick={(event) => {
                setMenuOpen(false);
                handleNavClick(event, "/#search");
              }}
            >
              Try search
            </Link>
          </div>
          <div className="raycast-navbar-actions-medium">
            <Link
              href="/#search"
              className="raycast-btn-light"
              onClick={(event) => {
                setMenuOpen(false);
                handleNavClick(event, "/#search");
              }}
            >
              Try search
            </Link>
          </div>
        </div>
      </div>
    </div>
  );

  if (!mounted) return null;

  return createPortal(navbar, document.body);
}
