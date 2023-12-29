import {get} from '@/utils/request'
export default {
    getWalkDistanceForTwoWeeks:()=>get('/data/summary/sum?dataType=walkingDistance&level=week&page=1&pageSize=2'),
    getRunDistanceForTwoWeeks:()=>get('/data/summary/sum?dataType=runningDistance&level=week&page=1&pageSize=2'),
    getDailyStep:()=>get('/data/summary/sum?dataType=stepCount&level=day&beginTime=2023-01-01T00:00:00&page=1&pageSize=366')
}