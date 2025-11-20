"use client";
import { ReactNode } from "react";

// export const metadata = {
//   title: "Auth",
//   description: "Authentication pages",
// };

export default function AuthLayout({ children }: { children: ReactNode }) {
  return (
    <div className="w-full min-h-screen flex items-center justify-center">
      {children}
    </div>
  );
}
