use run_time_data::RunTimeData;

pub struct ClassExecutor
{
    run_time_data: RunTimeData
}

impl ClassExecutor
{
    pub fn new() -> ClassExecutor
    {
        ClassExecutor
        {
            run_time_data: RunTimeData::new()
        }
    }
}