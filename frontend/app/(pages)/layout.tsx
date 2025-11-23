import "../globals.css";
import { SidebarInset, SidebarTrigger } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/Sidebar";
import { ModeToggle } from "@/components/ToggleTheme";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <main className="flex p-2 w-full rounded-lg">
      <ModeToggle />
      <AppSidebar />
      <SidebarTrigger />
      {children}
    </main>
  );
}
