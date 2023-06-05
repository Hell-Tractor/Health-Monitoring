import {get} from '@/utils/request'
export default {
    getHeartRate:()=>get('/data?dataType=heartRate&page=1&pageSize=1'),
    getBloodPressureLow:()=>get('/data?dataType=bloodPressureLow&page=1&pageSize=1'),
    getBloodPressureHigh:()=>get('/data?dataType=bloodPressureHigh&page=1&pageSize=1'),
    getBloodOxygen:()=>get('/data?dataType=bloodOxygen&page=1&pageSize=1'),
    getFloorNumByPeriod:()=>get('/data/summary/sum/interval/4?dataType=floorClimbed'),
    getStepNumByPeriod:()=>get('/data/summary/sum/interval/4?dataType=stepCount'),
    getWalkDistanceByPeriod:()=>get('/data/summary/sum/interval/4?dataType=walkingDistance'),
    getRunDistanceByPeriod:()=>get('/data/summary/sum/interval/4?dataType=runningDistance'),
    getHeartRateWarn:()=>get('/data/warn?dataType=heartRate'),
    getBloodPressureWarn:()=>get('/data/warn?dataType=bloodPressureHigh'),
    getBloodOxygenWarn:()=>get('/data/warn?dataType=bloodOxygen')
}