import SearchHome from "@/components/SearchHome";
import RaycastNavbar from "@/components/raycast/RaycastNavbar";

export default function Home() {
  return (
    <div className="raycast-site min-h-screen">
      <RaycastNavbar />
      <main>
        <SearchHome />
      </main>
    </div>
  );
}
