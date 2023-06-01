import {get} from '@/utils/request'
export default {
    getHeartRate:()=>get('/data_url/data?dataType=heartRate&page=1&pageSize=1'),
    getBloodPressureLow:()=>get('/data_url/data?dataType=bloodPressureLow&page=1&pageSize=1'),
    getBloodPressureHigh:()=>get('/data_url/data?dataType=bloodPressureHigh&page=1&pageSize=1'),
    getBloodOxygen:()=>get('/data_url/data?dataType=bloodOxygen&page=1&pageSize=1')
}