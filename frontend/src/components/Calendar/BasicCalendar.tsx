"use client";
import "react-big-calendar/lib/css/react-big-calendar.css";
import Calendar from '../Calendar';
// import "./index.css";
import moment from "moment";
import { EventItem } from '@/types/calendar';
import { allCongeType } from "@/schemas/schemaTable";
import React from "react";
import { AllConge } from "@/types/Employe";

// export const EVENTS: EventItem[] = [
//     {
//         start: moment("2024-08-10T10:00:00").toDate(),
//         end: moment("2024-08-20T11:00:00").toDate(),
//         title: "Hello",
//     },
//     {
//         start: moment("2024-08-10T12:00:00").toDate(),
//         end: moment("2024-08-11T13:00:00").toDate(),
//         title: "Faharoa ito"
//     },
//     {
//         start: moment("2024-08-13T10:00:00").toDate(),
//         end: moment("2024-08-13T13:00:00").toDate(),
//         title: "Fahatelo Ito"
//     },
// ];

type CalendarProps = {
    events: AllConge[];
}

export const MyCalendar: React.FC<CalendarProps> = ({ events }) => {

    const Events: EventItem[] = events.map((conge: AllConge) => ({
        start: moment(conge.date_deb_cong).toDate(),
        end: moment(conge.date_fin_cong).toDate(),
        title: conge.motif_cong,

        
    }));

    const handleSelectSlot = (slotInfo:any) => {
        console.log(slotInfo)
    }
    return (
    <>
        <Calendar events={Events}
            startAccessor="start"
            endAccessor="end"
            style={{ height: 500 }}
            selectable={true}
            onSelectSlot={handleSelectSlot} />
    </>
)
}