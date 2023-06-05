import {get} from '@/utils/request'
export default {
    getHeartRate:()=>get('/data_url/data?dataType=heartRate&page=1&pageSize=1'),
    getBloodPressureLow:()=>get('/data_url/data?dataType=bloodPressureLow&page=1&pageSize=1'),
    getBloodPressureHigh:()=>get('/data_url/data?dataType=bloodPressureHigh&page=1&pageSize=1'),
    getBloodOxygen:()=>get('/data_url/data?dataType=bloodOxygen&page=1&pageSize=1'),
    getFloorNumByPeriod:()=>get('/data_url/data/summary/sum/interval/4?dataType=floorClimbed'),
    getStepNumByPeriod:()=>get('/data_url/data/summary/sum/interval/4?dataType=stepCount'),
    getWalkDistanceByPeriod:()=>get('/data_url/data/summary/sum/interval/4?dataType=walkingDistance'),
    getRunDistanceByPeriod:()=>get('/data_url/data/summary/sum/interval/4?dataType=runningDistance'),
    getHeartRateWarn:()=>get('/data_url/data/warn?dataType=heartRate'),
    getBloodPressureWarn:()=>get('/data_url/data/warn?dataType=bloodPressureHigh'),
    getBloodOxygenWarn:()=>get('/data_url/data/warn?dataType=bloodOxygen')
}