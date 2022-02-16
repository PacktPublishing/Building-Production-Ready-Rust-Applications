use crate::data::stock::*;
use crate::data::*;

/// Complete the fulfilment of an order and updates the stock balance
/// # Arguments
///
/// * 'id' - Id of the Order being fulfilled
///
/// # Examples
/// let order = Order::new(id: 3,
///                 String::new("Aeroplanes Book"),
///                 32,
///                 5,
///                 4,
///                 String::new("4 Book Street, London"));
///            
/// result = complete_fulfill_order(order.id);
///
pub fn complete_fulfill_order(id: i32) -> Result<(), ()> {
    let connection = get_connection();

    // Get Order amount
    let order = orders::fulfill_order(&connection, id);

    let result = match order {
        Some(o) => {
            let stocks = get_stock(&connection, id);

            if stocks.len() == 1 {
                // if order amount is <= stock amount

                if o.amount <= stocks[0].amount {
                    //      Decrement stock amount by order amount
                    increment_stock(&connection, stocks[0].id, stocks[0].amount);

                    //      Set order to fulfilled.
                    orders::fulfill_order(&connection, o.id);
                }
            }
            Ok(())
        }
        None => Err(()),
    };

    result
}
