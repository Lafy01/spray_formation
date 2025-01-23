use chrono::{Datelike, Duration, NaiveDateTime, Timelike, Weekday};

pub fn calculer_date_fin_conge(date_deb_cong: NaiveDateTime, nb_jour_cong: f64) -> NaiveDateTime {
    let mut date_fin_cong = date_deb_cong;
    let mut remaining_days = nb_jour_cong;

    // Gestion de la demi-journée dès le départ
    if remaining_days >= 0.5 && remaining_days < 1.0 {
        if date_deb_cong.hour() < 12 {
            date_fin_cong = date_fin_cong.date().and_hms_opt(12, 0, 0).unwrap();
        } else {
            date_fin_cong = date_fin_cong.date().and_hms_opt(18, 0, 0).unwrap();
        }
        remaining_days = 0.0; // Le congé est terminé après la demi-journée
    } else if remaining_days >= 1.0 {
        // Si on a au moins un jour complet, on décrémente d'abord 0.5 pour compter la première demi-journée
        remaining_days -= 0.5; 
        
        // On se positionne à la fin de la première demi-journée
        if date_deb_cong.hour() < 12 {
            date_fin_cong = date_fin_cong.date().and_hms_opt(12, 0, 0).unwrap();
        } else {
            date_fin_cong = date_fin_cong.date().and_hms_opt(18, 0, 0).unwrap();
        }
    }

    while remaining_days > 0.0 {
        date_fin_cong += Duration::days(1); 

        // Vérification du week-end
        if date_fin_cong.weekday() == Weekday::Sat || date_fin_cong.weekday() == Weekday::Sun {
            continue; // Passer au jour suivant si c'est un week-end
        }

        remaining_days -= 1.0; // Décrémenter uniquement si c'est un jour de semaine
    }

    date_fin_cong
}





// use chrono::{Datelike, Duration, NaiveDateTime, Timelike, Weekday};


// pub fn calculer_date_fin_conge(date_deb_cong: NaiveDateTime, nb_jour_cong: f64) -> NaiveDateTime {
//     let mut date_fin_cong = date_deb_cong;
//     let mut remaining_days = nb_jour_cong;

//     while remaining_days > 0.0 {
//         date_fin_cong += Duration::days(1);

//         // Vérification du week-end
//         if date_fin_cong.weekday() == Weekday::Sat || date_fin_cong.weekday() == Weekday::Sun {
//             continue; // Passer au jour suivant si c'est un week-end
//         }

//         remaining_days -= 1.0; // Décrémenter uniquement si c'est un jour de semaine

//         // Gestion de la demi-journée (même logique qu'avant)
//         if remaining_days < 1.0 && remaining_days > 0.0 {
//             if date_deb_cong.hour() < 12 {
//                 date_fin_cong = date_fin_cong.date().and_hms_opt(12, 0, 0).unwrap();
//             } else {
//                 date_fin_cong = date_fin_cong.date().and_hms_opt(18, 0, 0).unwrap();
//             }
//             remaining_days = 0.0; // Assurer la fin de la boucle après la gestion de la demi-journée
//         }
//     }

//     date_fin_cong
// }

// pub fn calculer_date_fin_conge(
//     date_deb_cong: NaiveDateTime, 
//     nb_jour_cong: f64) 
// -> NaiveDateTime {
//     let mut date_fin_cong = date_deb_cong;
//     let full_days = nb_jour_cong.floor() as i64;
//     let half_day = nb_jour_cong - nb_jour_cong.floor();

//     // Add full days
//     date_fin_cong += Duration::days(full_days);

//     // Add half day if applicable
//     if half_day > 0.0 {
//         if date_deb_cong.hour() < 12 {
//             // Started in the morning
//             if full_days == 0 {
//                 date_fin_cong = date_deb_cong.date().and_hms_opt(12, 0, 0).unwrap();
//             } else {
//                 date_fin_cong = date_fin_cong.date().and_hms_opt(12, 0, 0).unwrap();
//             }
//         } else {
//             // Started in the afternoon or after full days
//             if full_days == 0 {
//                 date_fin_cong = date_deb_cong.date().and_hms_opt(18, 0, 0).unwrap();
//             } else {
//                 date_fin_cong = date_fin_cong.date().and_hms_opt(18, 0, 0).unwrap();
//             }
//         }
//     }

//     date_fin_cong
// }