import "../globals.css";
import { SidebarTrigger } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/Sidebar";
import { ModeToggle } from "@/components/ToggleTheme";


export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <main className="flex p-2 w-full">
      <AppSidebar />
      <SidebarTrigger />
      <ModeToggle />
      {children}
    </main>
  );
}
