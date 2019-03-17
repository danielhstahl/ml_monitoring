const {
    createInitialInputMetrics,
    updateInputMetrics,
    generateConfusionMatrix, //for testing
    specificity,
    sensitivity,
    precision,
    accuracy,
}=require('../js/metrics')
describe('createInitialInputMetrics', ()=>{
    it('returns array of numbers based on keys with single element', ()=>{
        const inputArray=[{hello:5, goodbye:3}]
        const result=createInitialInputMetrics(inputArray, 5)
        expect(result).toEqual({hello:[5], goodbye:[3]})
    })
    it('returns array of numbers based on keys with multipe elements', ()=>{
        const inputArray=[{hello:5, goodbye:3},{hello:3, goodbye:5}]
        const result=createInitialInputMetrics(inputArray, 5)
        expect(result).toEqual({hello:[5, 3], goodbye:[3, 5]})
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
        expect(result2).toEqual({hello:[5, 5], goodbye:[3, 3]})
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


