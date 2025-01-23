export type Appointment = {
    id: number;
    status: string;
    location: string;
    resource: string;
    address: string;
};

export type Blockout = { id: number; name: string };

export type EventItem = {
    start?: Date;
    end?: Date;
    title?: String;
};
