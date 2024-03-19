// @generated automatically by Diesel CLI.
#![allow(non_snake_case)]

diesel::table! {
    AirPlaneSeatTypes (id_placa_type) {
        id_placa_type -> Int4,
        type_name -> Text,
    }
}

diesel::table! {
    AirPlanes (id_plane) {
        id_plane -> Int4,
        plane_name -> Text,
    }
}

diesel::table! {
    AirPorts (id_airport) {
        id_airport -> Int4,
        airport_name -> Text,
        airport_address -> Text,
    }
}

diesel::table! {
    Clients (id_client) {
        id_client -> Int4,
        name -> Text,
        day_of_birth -> Date,
        mail -> Text,
        passport -> Text,
    }
}

diesel::table! {
    FlightsSchedule (id_flight) {
        id_flight -> Int4,
        id_plane -> Int4,
        from_id_airport -> Int4,
        to_id_airport -> Int4,
        depature_date -> Timestamp,
        arrive_date -> Timestamp,
    }
}

diesel::table! {
    PlaneSeatConnections (id_plane_seat) {
        id_plane_seat -> Int4,
        id_place_type -> Int4,
        id_plane -> Int4,
        seat_number -> Int4,
    }
}

diesel::table! {
    ReceiptType (id_receipt_type) {
        id_receipt_type -> Int4,
        type_name -> Text,
    }
}

diesel::table! {
    Receipts (id_receipt) {
        id_receipt -> Int4,
        id_ticket -> Int4,
        date_order -> Timestamp,
        id_receipt_type -> Int4,
    }
}

diesel::table! {
    Tickets (id_ticket) {
        id_ticket -> Int4,
        id_flight -> Int4,
        id_client -> Int4,
        seat_number -> Int4,
        baggage -> Bool,
        value_wo_b -> Float8,
        value_w_b -> Float8,
    }
}

diesel::table!
{
	Users (id_user){
		id_user -> Int4,
		name -> Text,
		password -> Text,
		id_user_type -> Int4,
	}
}

diesel::table!
{
	UserTypes (id_user_type){
		id_user_type -> Int4,
		type_name -> Text,
	}
}


diesel::joinable!(FlightsSchedule -> AirPlanes (id_plane));
diesel::joinable!(PlaneSeatConnections -> AirPlaneSeatTypes (id_place_type));
diesel::joinable!(PlaneSeatConnections -> AirPlanes (id_plane));
diesel::joinable!(Tickets -> Clients (id_client));
diesel::joinable!(Tickets -> FlightsSchedule (id_flight));
diesel::joinable!(Users -> UserTypes(id_user_type));


diesel::allow_tables_to_appear_in_same_query!(
    AirPlaneSeatTypes,
    AirPlanes,
    AirPorts,
    Clients,
    FlightsSchedule,
    PlaneSeatConnections,
    ReceiptType,
    Receipts,
    Tickets,
	UserTypes,
	Users,
);
