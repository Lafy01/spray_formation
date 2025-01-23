"use client"

import { MyCalendar } from "@/components/Calendar/BasicCalendar";
import { useAllMyConge } from "@/hooks/useConge";

export default function CalendrierPage() {
  const { myConge } = useAllMyConge();

  const congeDisplay = myConge.filter(
    (conge) => conge.status_cong != "Refuser"
  )

  return (
    <div className=" h-screen flex-1 flex-col space-y-8 p-8 md:flex">
      HEllo calendar
      <MyCalendar events={congeDisplay} />
    </div>
  );
}
