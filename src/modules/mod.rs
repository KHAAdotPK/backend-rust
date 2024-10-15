/*
    src/modules/mod.rs
    aEon@khaa.pk
 */

/*
    The pub(crate) visibility specifier allows modules to be used within the same crate while restricting their use from external crates.
 */

/*
    The pub(crate) visibility specifier means that the module "constants" can be accessed by other code within the same crate (i.e., your project), but it's not public to external code.
 */
pub(crate) mod constants;

/*
    This line defines another module named "khaa_pk_read_write" within the parent module. Similar to the "constants" module, it is also visible to code within the same crate, but not accessible from external code.
 */
pub(crate) mod khaa_pk_read_write;

/*
    This line defines yet another module named "model" within the parent module. It follows the same visibility pattern as the previous modules.
 */
pub(crate) mod model; 
