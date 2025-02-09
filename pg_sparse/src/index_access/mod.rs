use pgrx::*;

mod build;
mod cost;
mod delete;
mod insert;
pub mod options;
mod scan;
mod vacuum;
mod validate;

#[pg_extern(sql = "
CREATE FUNCTION sparse_hnsw_handler(internal) RETURNS index_am_handler LANGUAGE c AS 'MODULE_PATHNAME', '@FUNCTION_NAME@';
CREATE ACCESS METHOD sparse_hnsw TYPE INDEX HANDLER sparse_hnsw_handler;
COMMENT ON ACCESS METHOD sparse_hnsw IS 'sparse index access method';
")]
fn sparse_hnsw_handler(_fcinfo: pg_sys::FunctionCallInfo) -> PgBox<pg_sys::IndexAmRoutine> {
    let mut amroutine =
        unsafe { PgBox::<pg_sys::IndexAmRoutine>::alloc_node(pg_sys::NodeTag::T_IndexAmRoutine) };

    amroutine.amstrategies = 0;
    amroutine.amsupport = 1;
    amroutine.amcanorder = false;
    amroutine.amcanorderbyop = true;
    amroutine.amcanbackward = false;
    amroutine.amcanunique = false;
    amroutine.amcanmulticol = false;
    amroutine.amoptionalkey = true;
    amroutine.amsearcharray = false;
    amroutine.amsearchnulls = false;
    amroutine.amstorage = false;
    amroutine.amclusterable = false;
    amroutine.ampredlocks = false;
    amroutine.amcanparallel = false;
    amroutine.amcaninclude = false;

    amroutine.amcanreturn = None;
    amroutine.amoptions = None;
    amroutine.amproperty = None;
    amroutine.ammarkpos = None;
    amroutine.amrestrpos = None;
    amroutine.amestimateparallelscan = None;
    amroutine.aminitparallelscan = None;
    amroutine.amparallelrescan = None;

    amroutine.amkeytype = pg_sys::InvalidOid;

    amroutine.amvalidate = Some(validate::amvalidate);
    amroutine.ambuild = Some(build::ambuild);
    amroutine.ambuildempty = Some(build::ambuildempty);
    amroutine.aminsert = Some(insert::aminsert);
    amroutine.ambulkdelete = Some(delete::ambulkdelete);
    amroutine.amvacuumcleanup = Some(vacuum::amvacuumcleanup);
    amroutine.amcostestimate = Some(cost::amcostestimate);
    amroutine.amoptions = Some(options::amoptions);
    amroutine.ambeginscan = Some(scan::ambeginscan);
    amroutine.amrescan = Some(scan::amrescan);
    amroutine.amgettuple = Some(scan::amgettuple);
    amroutine.amendscan = Some(scan::amendscan);
    amroutine.amgetbitmap = None;

    #[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
    {
        amroutine.amoptsprocnum = 0;
        amroutine.amusemaintenanceworkmem = false;
        amroutine.amparallelvacuumoptions = pg_sys::VACUUM_OPTION_PARALLEL_BULKDEL as u8;
        amroutine.ambuildphasename = None;
    }

    #[cfg(any(feature = "pg14", feature = "pg15", feature = "pg16"))]
    {
        amroutine.amadjustmembers = None;
    }

    amroutine.into_pg_boxed()
}
