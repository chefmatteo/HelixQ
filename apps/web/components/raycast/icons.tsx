export function HelixQLogoMark({ className = "" }: { className?: string }) {
  return (
    <svg
      className={`helixq-logo-mark ${className}`}
      viewBox="0 0 32 32"
      width="24"
      height="24"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
    >
      <defs>
        <linearGradient id="helixq-sigma-fill" x1="16" y1="2" x2="16" y2="30" gradientUnits="userSpaceOnUse">
          <stop stopColor="#ffffff" />
          <stop offset="1" stopColor="#7ad0ff" />
        </linearGradient>
      </defs>
      <text
        x="16"
        y="24"
        textAnchor="middle"
        fill="url(#helixq-sigma-fill)"
        style={{
          fontFamily: "Georgia, 'Times New Roman', 'Noto Serif', serif",
          fontSize: "28px",
          fontWeight: 500,
          letterSpacing: "-0.04em",
        }}
      >
        ∑
      </text>
    </svg>
  );
}

export function AppleIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 16 16" aria-hidden="true">
      <path
        fill="currentColor"
        d="M12.665 15.358c-.905.844-1.893.711-2.843.311-1.006-.409-1.93-.427-2.991 0-1.33.551-2.03.391-2.825-.31C-.498 10.886.166 4.078 5.28 3.83c1.246.062 2.114.657 2.843.71 1.09-.213 2.133-.826 3.296-.746 1.393.107 2.446.64 3.138 1.6-2.88 1.662-2.197 5.315.443 6.337-.526 1.333-1.21 2.657-2.345 3.635zM8.03 3.778C7.892 1.794 9.563.16 11.483 0c.268 2.293-2.16 4-3.452 3.777"
      />
    </svg>
  );
}
