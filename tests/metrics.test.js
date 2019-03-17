const {
    computeDrift,
    createInitialInputMetrics,
    updateInputMetrics,
    generateConfusionMatrix, //for testing
    specificity,
    sensitivity,
    precision,
    accuracy,
    //getDriftForEachVariable,
}=require('../js/metrics')
describe('computeDrift', ()=>{
    it('returns a number', ()=>{
        const x=[1, 2, 3, 4, 5, 6, 7]
        const y=[1, 2, 3, 4, 5, 6, 8]
        expect(computeDrift(x, y)).toBeDefined()
    })
})/*
describe('getDriftForEachVariable', ()=>{
    it('gets number for each attribute', ()=>{
        const inputArray=[{hello:5, goodbye:3}]
        const result=createInitialInputMetrics(inputArray, 5)
        const keys=["hello", "goodbye"]
        let result2=updateInputMetrics(keys, result, inputArray, 5)
        result2=updateInputMetrics(keys, result2, inputArray, 5)
        result2=updateInputMetrics(keys, result2, inputArray, 5)
        result2=updateInputMetrics(keys, result2, inputArray, 5)
        const result3=getDriftForEachVariable(result2, [3, 4, 5, 6, 6, 7, 8, 9])
        expect(result3.hello).toBeDefined()
        expect(result3.goodbye).toBeDefined()
    })
})*/
describe('createInitialInputMetrics', ()=>{
    it('returns array of numbers based on keys with single element', ()=>{
        const inputArray=[{hello:5, goodbye:3}]
        const result=createInitialInputMetrics(inputArray, 5)
        expect(result).toEqual({hello:{data:[5], drift:0.3}, goodbye:{data:[3], drift:0.4}})
    })
    it('returns array of numbers based on keys with multipe elements', ()=>{
        const inputArray=[{hello:5, goodbye:3},{hello:3, goodbye:5}]
        const result=createInitialInputMetrics(inputArray, 5)
        expect(result).toEqual({hello:{data:[5, 3], drift:0.3}, goodbye:{data:[3, 5], drift:0.3}})
    })
    it('errors with array with no elements', ()=>{
        const inputArray=[]
        expect(()=>createInitialInputMetrics(inputArray, 5)).toThrow()
    })
})
describe('updateInputMetrics', ()=>{
    it('updates array of numbers based on keys with single element', ()=>{
        const inputArray=[{hello:5, goodbye:3}]
        const result=createInitialInputMetrics(inputArray, 5)
        const keys=["hello", "goodbye"]
        const result2=updateInputMetrics(keys, result, inputArray, 5)
        expect(result2).toEqual({hello:{data:[5, 5], drift:0.3}, goodbye:{data:[3, 3], drift:0.4}})
    })
})
describe('generateConfusionMatrix', ()=>{
    it('generates confusion matrix from array 1', ()=>{
        const arr=[
            {predicted:1, actual:1}, 
            {predicted:0, actual:1},
            {predicted:0, actual:0},
            {predicted:1, actual:0},
        ]
        const expected={tp:1, tn:1, fn:1, fp:1}
        expect(generateConfusionMatrix(arr)).toEqual(expected)
    })
    it('generates confusion matrix from array 2', ()=>{
        const arr=[{predicted:1, actual:1}, {predicted:0, actual:1}]
        const expected={tp:1, tn:0, fn:1, fp:0}
        expect(generateConfusionMatrix(arr)).toEqual(expected)
    })
    it('generates confusion matrix from array 3', ()=>{
        const arr=[{predicted:1, actual:0}, {predicted:0, actual:1}]
        const expected={tp:0, tn:0, fn:1, fp:1}
        expect(generateConfusionMatrix(arr)).toEqual(expected)
    })
})
describe('metrics', ()=>{
   //from wikipedia
    const cfm={tp:20, tn:1820, fn:10, fp:180}
    it('correctly computes specificity', ()=>{
        const result=specificity(cfm)
        expect(result).toEqual(1820 / (180 + 1820))
    })
    it('correctly computes sensitivity', ()=>{
        const result=sensitivity(cfm)
        expect(result).toEqual(20 / (20 + 10))
    })
    it('correctly computes precision', ()=>{
        const result=precision(cfm)
        expect(result).toEqual(20 / (20 + 180))
    })
    it('correctly computes accuract', ()=>{
        const result=accuracy(cfm)
        expect(result).toEqual(1840/(1850+180))
    })
})


