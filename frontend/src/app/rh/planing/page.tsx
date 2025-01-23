"use client";
import { SchedulerPlaning } from "@/components/Scheduler";
import { useAllEmployePlanning } from "@/hooks/useAllEmploye";
import { SchedulerData } from "@bitnoi.se/react-scheduler";

export default function PlanningPage() {
  const { myEmployes } = useAllEmployePlanning();

  const congeEmploye: SchedulerData = myEmployes.map((employe) => ({
    id: employe.id_empl,
    label: {
      icon: "",
      title: `${employe.nom_empl} ${employe.prenom_empl}`,
      subtitle: employe.role,
    },
    data: employe.conge.filter((conge) => !conge.status_cong.includes("Refuser")).map((conge) => ({
      id: conge.id_cong,
      startDate: new Date(conge.date_deb_cong),
      endDate: new Date(conge.date_fin_cong),
      title: conge.motif_cong,
      occupancy: Number(conge.nb_jour_cong),
      bgColor: "rgb(114, 141, 226)",
      description: conge.motif_cong,
      subtitle: conge.motif_cong
    }))
  }))
  console.log(congeEmploye)
  return (
    <div>
      <h1  >Planning</h1>
      <div className="relative w-full h-[700px]">

      <SchedulerPlaning conge={congeEmploye}/>
      </div>
    </div>
  );
}
