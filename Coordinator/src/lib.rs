
#![no_std]
use gstd::{errors::Result, msg , prelude::*,ActorId};
use gmeta::{Metadata};
use hashbrown::HashMap;
use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));



static mut STATE:Option<HashMap<ActorId, u128>> = None;



fn state_mut() -> &'static mut HashMap<ActorId,u128> {

    let state = unsafe { STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }


}


#[no_mangle]
extern "C" fn init () {

   unsafe { STATE = Some(HashMap::new())}


}

#[no_mangle]
extern "C" fn handle(){


    handle_state().expect("Execution Error")


}

    

fn handle_state() -> Result<()> {
    // Cargamos el mensaje de entrada
    let payload = msg::load()?;

    match payload {
        Action::ExampleAction => {
            let current_state = state_mut();
            // Actualiza el estado como lo necesites, por ejemplo:
            current_state.insert(msg::source(), 10);
            // Genera un mensaje de respuesta
            msg::reply(Event::ExampleEvent, 0)?;
        }
        Action::RegisterUser(actor_id) => {
            let current_state = state_mut();
            // Verifica si el usuario ya está registrado
            if current_state.contains_key(&actor_id) {
                // El usuario ya está registrado, maneja el error si es necesario
                // Puedes devolver un error o enviar una respuesta personalizada
                // msg::reply(Error::new(0, "El usuario ya está registrado"), 0)?;
                // O puedes simplemente retornar Ok(()) si no deseas manejar el error
                msg::reply(Error::new(0, "El usuario ya está registrado"), 0)?;
                return Ok(());
            }
            // Registra al usuario agregando su ActorId al estado
            current_state.insert(actor_id, 0); // Puedes inicializarlo con un valor específico si es necesario
            // Emite el evento de registro de usuario
            msg::reply(Event::UserRegistered(actor_id), 0)?;
        }
        Action::AddItemWeapon => {
            // Lógica para manejar la acción AddItemWeapon
            // Puedes implementar esta parte según tus necesidades
        }
        Action::AddItemArmor => {
            // Lógica para manejar la acción AddItemArmor
            // Puedes implementar esta parte según tus necesidades
        }
        _ => {
            // Manejar otras acciones aquí si es necesario
        }
    }

    Ok(())
}



    #[no_mangle]
    extern "C" fn state() {

        // We create a state variable.
        let state: <ContractMetadata as Metadata>::State =
            state_mut().iter().map(|(k, v)| (*k, *v)).collect();
         
        // Generate response message
        msg::reply(state, 0).expect("failed to encode or reply from `state()`");
    }
