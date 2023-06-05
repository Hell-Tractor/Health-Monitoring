import {get} from '@/utils/request'
export default {
    getWalkDistanceForTwoWeeks:()=>get('/data/summary/sum?dataType=walkingDistance&level=week&page=1&pageSize=2'),
    getRunDistanceForTwoWeeks:()=>get('/data/summary/sum?dataType=runningDistance&level=week&page=1&pageSize=2'),
    getDailyStep:()=>get('/data/summary/sum?dataType=stepCount&level=day&page=1&pageSize=366')
}